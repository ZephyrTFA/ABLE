use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    if std::env::var("DATABASE_URL").is_err() {
        std::env::set_var(
            "DATABASE_URL",
            "mysql://library:library@localhost:3306/library",
        );
    }
    cli::run_cli(migration::Migrator).await;
}
