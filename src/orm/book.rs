use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

pub type Book = Model;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "book")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub title: String,
    pub author: String,
    pub publication_year: u64,
    pub isbn: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
