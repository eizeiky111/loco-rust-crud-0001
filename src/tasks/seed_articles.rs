use crate::models::_entities::articles::ActiveModel;
use chrono::{DateTime, FixedOffset, Utc};
use loco_rs::prelude::*;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng; // To seed the StdRng

pub struct SeedArticles;

#[async_trait]
impl Task for SeedArticles {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "seed_articles".to_string(),
            detail: "Task to seed articles with dummy data".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        println!("Task SeedArticles started");

        // Use StdRng which is Send
        let mut rng = StdRng::from_entropy();
        let num_articles = 10; // Number of articles to seed

        for _ in 0..num_articles {
            let title = format!("Dummy Article {}", rng.gen_range(1..=100));
            let content = format!("This is the content of {title}");

            let now: DateTime<FixedOffset> = Utc::now().with_timezone(
                &FixedOffset::east_opt(999).unwrap_or(FixedOffset::east_opt(0).unwrap()),
            );

            let item = ActiveModel {
                created_at: ActiveValue::Set(now),
                updated_at: ActiveValue::Set(now),
                title: ActiveValue::Set(Some(title)),
                content: ActiveValue::Set(Some(content)),
                ..Default::default()
            };

            item.insert(&app_context.db).await?;
        }

        println!("Seeded {} articles", num_articles);
        Ok(())
    }
}
