use sea_orm::DatabaseConnection;

pub struct DatabaseContainer(DatabaseConnection);

impl DatabaseContainer {
    pub fn get(&self) -> &DatabaseConnection {
        &self.0
    }

    pub fn new(connection: DatabaseConnection) -> Self {
        Self(connection)
    }
}
