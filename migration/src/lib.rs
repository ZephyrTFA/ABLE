pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table_book;
mod m20220101_000002_create_table_user;
mod m20220101_000003_create_table_permissions;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table_book::Migration),
            Box::new(m20220101_000002_create_table_user::Migration),
            Box::new(m20220101_000003_create_table_permissions::Migration),
        ]
    }
}

#[derive(Iden)]
pub enum Book {
    Table,
    Id,
    Title,
    Author,
    PublicationYear,
    Isbn,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Username,
    Salt,
    Hash,
    Enabled,
    CreatedAt,
    Token,
    TokenExpiry,
    PermissionId,
}

#[derive(Iden)]
pub enum Permissions {
    Table,
    Id,
    User,
    #[allow(clippy::enum_variant_names)]
    Permissions,
}
