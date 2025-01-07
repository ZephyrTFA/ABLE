use std::{mem::take, sync::Mutex};

use ::mysql::Pool;
use log::trace;
use mysql::{Opts, PooledConn};
use once_cell::sync::Lazy;

#[derive(Default, Debug)]
pub struct DatabaseHelper {
    connection: Option<Pool>,
    connection_string: String,
}

static DATABASE: Lazy<Mutex<DatabaseHelper>> = Lazy::new(|| Mutex::new(DatabaseHelper::default()));
impl DatabaseHelper {
    pub fn get_connection() -> Result<PooledConn, String> {
        trace!("getting db pooled connection");
        let mut db = DATABASE.lock().map_err(|e| e.to_string())?;
        if db.connection.is_none() {
            db.connect()?;
        }
        db.connection
            .as_ref()
            .unwrap()
            .get_conn()
            .map_err(|e| e.to_string())
    }

    pub fn init(connection_string: String) -> Result<(), String> {
        let mut db: std::sync::MutexGuard<'_, DatabaseHelper> =
            DATABASE.lock().map_err(|e| e.to_string())?;

        if db.connection.is_some() {
            take(&mut db.connection);
        }
        db.connection_string = connection_string;
        db.connect()?;
        Ok(())
    }

    fn connect(&mut self) -> Result<(), String> {
        if self.connection.is_some() {
            take(&mut self.connection);
        }
        let options = Opts::from_url(&self.connection_string).map_err(|e| e.to_string())?;
        let pool = Pool::new(options).map_err(|e| e.to_string())?;
        self.connection = Some(pool);
        Ok(())
    }
}
