use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Repository {
    pub id: sqlx::types::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: sqlx::types::Uuid,
    pub visibility: String,
}
