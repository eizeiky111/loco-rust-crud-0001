use super::_entities::comments::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
pub type Comments = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
