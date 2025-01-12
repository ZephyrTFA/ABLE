use sea_orm_migration::{prelude::*, schema::*};

use crate::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        integer_uniq(User::Id)
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(string(User::Username).not_null().unique_key())
                    .col(string(User::Salt).not_null())
                    .col(string(User::Hash))
                    .col(boolean(User::Enabled).not_null())
                    .col(timestamp(User::CreatedAt).not_null())
                    .col(string(User::Token))
                    .col(timestamp(User::TokenExpiry))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}
