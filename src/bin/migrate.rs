use sitebuilder::db;

#[tokio::main]
async fn main() -> Result<(), db::DbError> {
    let pool = db::get_pool().await?;

    sqlx::migrate!()
        .run(&pool)
        .await?;

    Ok(())
}
