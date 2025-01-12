use sea_orm_migration::{prelude::*, schema::*};

use crate::{Permissions, User};

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
                    .col(
                        integer_uniq(Permissions::Id)
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(integer_uniq(Permissions::User).not_null())
                    .col(integer(Permissions::Permissions).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_permissions_user_id")
                            .from(User::Table, User::Id)
                            .to(Permissions::Table, Permissions::User)
                            .on_update(ForeignKeyAction::Cascade)
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
