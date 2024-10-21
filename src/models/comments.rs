use sea_orm::entity::prelude::*;
use super::_entities::comments::{ActiveModel, Entity};
pub type Comments = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
