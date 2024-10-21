use super::_entities::articles::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
pub type Articles = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
