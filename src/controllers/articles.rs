#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::{
    articles::{ActiveModel, Entity, Model},
    comments,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Serialize)]
struct ListResponse {
    data: Vec<Params>, // Assuming Params has fields 'title' and 'content'
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.title = Set(self.title.clone());
        item.content = Set(self.content.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}

pub async fn list_b(State(ctx): State<AppContext>) -> Result<Response> {
    // Fetch all items from the database
    let items = Entity::find().all(&ctx.db).await?;

    // Map the items to Params (or whatever your model struct is)
    let response_data: Vec<Params> = items
        .into_iter()
        .map(|item| Params {
            id: item.id,
            title: item.title,
            content: item.content,
        })
        .collect();

    // Create the ListResponse
    let response = ListResponse {
        data: response_data,
    };

    // Serialize and return the response
    format::json(response)
}

pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item: Model = item.insert(&ctx.db).await?;
    format::json(item)
}

pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}

pub async fn comments(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let comments = item.find_related(comments::Entity).all(&ctx.db).await?;
    format::json(comments)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("articles")
        .add("/", get(list))
        .add("/custom/list", get(list_b))
        .add("/", post(add))
        .add("/:id", get(get_one))
        .add("/:id", delete(remove))
        .add("/:id", post(update))
        .add("/:id/comments", get(comments))
}
