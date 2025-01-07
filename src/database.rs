use std::sync::Mutex;

use once_cell::sync::OnceCell;
use sea_orm::{Database, DatabaseConnection};

static DATABASE_POOL: OnceCell<Mutex<DatabaseConnection>> = OnceCell::new();

pub async fn init(connection_string: &str) -> Result<(), String> {
    if DATABASE_POOL.get().is_none() {
        let new_pool = Mutex::new(
            Database::connect(connection_string)
                .await
                .map_err(|e| e.to_string())?,
        );
        DATABASE_POOL
            .set(new_pool)
            .map_err(|_| "already init".to_string())?;
    }
    Ok(())
}
