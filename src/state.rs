use crate::library::Library;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState(Library, DatabaseConnection);

impl AppState {
    pub fn library(&self) -> &Library {
        &self.0
    }
    pub fn library_mut(&mut self) -> &mut Library {
        &mut self.0
    }

    pub fn db(&self) -> DatabaseConnection {
        self.1.clone()
    }

    pub fn new(library: Library, db: DatabaseConnection) -> Self {
        Self(library, db)
    }
}

pub fn create_state(db_connection: DatabaseConnection) -> AppState {
    AppState::new(Library::default(), db_connection)
}
