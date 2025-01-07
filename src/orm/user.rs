use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

pub type User = Model;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub salt: String,
    pub hash: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
