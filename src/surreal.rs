use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

use crate::configuration::DatabaseSettings;

pub async fn get_db(config: DatabaseSettings) -> surrealdb::Result<crate::Db> {
    let db = Surreal::new::<Ws>(config.connection_string()).await?;

    db.signin(Root {
        username: &config.username,
        password: &config.password,
    })
    .await?;

    db.use_ns("test").use_db(&config.database_name).await?;
    Ok(db)
}
