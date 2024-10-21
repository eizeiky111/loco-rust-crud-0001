use sea_orm::entity::prelude::*;
use super::_entities::articles::{ActiveModel, Entity};
pub type Articles = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
