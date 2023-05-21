use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApplicatonSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub database_name: String,
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Configuration {
    pub database: DatabaseSettings,
    pub application: ApplicatonSettings,
}

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new("config.yaml", config::FileFormat::Yaml))
        .build()?;
    settings.try_deserialize::<Configuration>()
}
