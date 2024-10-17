use sqlx::postgres::{PgPoolOptions, PgPool};
use sqlx::migrate::MigrateError;
use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum DbError {
    EnvVarError(std::env::VarError),
    SqlError(SqlxError),
}

impl From<std::env::VarError> for DbError {
    fn from(value: std::env::VarError) -> Self {
        Self::EnvVarError(value)
    }
}

impl From<SqlxError> for DbError {
    fn from(value: SqlxError) -> Self {
        Self::SqlError(value)
    }
}

impl From<MigrateError> for DbError {
    fn from(value: MigrateError) -> Self {
        Self::SqlError(SqlxError::Migrate(Box::new(value)))
    }
}

pub async fn get_pool() -> Result<PgPool, DbError>  {
    let url = std::env::var("DATABASE_URL")?;
    Ok(PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?)
}
