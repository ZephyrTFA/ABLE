use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Permissions::Table)
                    .if_not_exists()
                    .col(pk_auto(Permissions::Id))
                    .col(pk_auto(Permissions::User))
                    .col(integer(Permissions::Permissions))
                    .foreign_key(
                        ForeignKey::create()
                            .from(User::Table, User::Id)
                            .to(Permissions::Table, Permissions::User)
                            .name("FK_permissions_user_id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(integer(User::PermissionId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Permissions::Table).to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::PermissionId)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Permissions {
    Table,
    Id,
    User,
    #[allow(clippy::enum_variant_names)]
    Permissions,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    PermissionId,
}
