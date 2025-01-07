use sea_orm::DatabaseConnection;

use crate::{auth::UserAuthentication, library::Library};

#[derive(Clone)]
pub struct AppState(Library, UserAuthentication, DatabaseConnection);

impl AppState {
    pub fn library(&self) -> &Library {
        &self.0
    }
    pub fn library_mut(&mut self) -> &mut Library {
        &mut self.0
    }

    pub fn auth(&self) -> &UserAuthentication {
        &self.1
    }
    pub fn auth_mut(&mut self) -> &mut UserAuthentication {
        &mut self.1
    }

    pub fn db(&self) -> DatabaseConnection {
        self.2.clone()
    }

    pub fn new(library: Library, auth: UserAuthentication, db: DatabaseConnection) -> Self {
        Self(library, auth, db)
    }
}

pub fn create_state(db_connection: DatabaseConnection) -> AppState {
    AppState::new(
        Library::default(),
        UserAuthentication::default(),
        db_connection,
    )
}
