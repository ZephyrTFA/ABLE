use sea_orm_migration::{prelude::*, schema::*};

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
                    .col(pk_auto(Book::Id))
                    .col(string(Book::Title))
                    .col(string(Book::Author))
                    .col(integer(Book::PublicationYear))
                    .col(string(Book::Isbn))
                    .col(timestamp(Book::CreatedAt))
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

#[derive(Iden)]
enum Book {
    Table,
    Id,
    Title,
    Author,
    PublicationYear,
    Isbn,
    CreatedAt,
    UpdatedAt,
}
