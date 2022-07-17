use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database: SQLite3Settings,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct SQLite3Settings {
    pub connection: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.toml",
            config::FileFormat::Toml,
        ))
        .build()?;
    settings.try_deserialize()
}
