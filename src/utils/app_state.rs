use sea_orm::DatabaseConnection;

#[allow(dead_code)]
pub struct AppState {
    pub db: DatabaseConnection,
}
