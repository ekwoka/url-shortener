use surrealdb::{engine::local::Mem, Surreal};

use crate::configuration::DatabaseSettings;

pub async fn get_db(config: DatabaseSettings) -> surrealdb::Result<crate::Db> {
    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns("test").use_db(&config.database_name).await?;
    Ok(db)
}
