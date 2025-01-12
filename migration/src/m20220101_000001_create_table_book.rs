use sea_orm_migration::{prelude::*, schema::*};

use crate::Book;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Book::Table)
                    .if_not_exists()
                    .col(
                        integer_uniq(Book::Id)
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(string(Book::Title).not_null())
                    .col(string(Book::Author).not_null())
                    .col(integer(Book::PublicationYear).not_null())
                    .col(string(Book::Isbn).not_null())
                    .col(timestamp(Book::CreatedAt).not_null())
                    .col(timestamp(Book::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Book::Table).to_owned())
            .await
    }
}
