#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
    let mut settings = config::Config::default();
    // Add configuration values from a file named `configuration`. // It will look for any top-level file with an extension
    // that `config` knows how to parse: yaml, json, etc. settings.merge(config::File::with_name("configuration"))?;
    // Try to convert the configuration values it read into Settings type and return Result wrapper
    settings.merge(config::File::with_name("configuration"))?;
    settings.try_into()
}
// allows our struct to have a function. Usage: impl DatabaseSettings for Settings{
// 
//}
impl DatabaseSettings {
    pub fn connection_string(&self) -> String { format!(
        // connection string formatting with username, password, and more info for sucessful DB connect
                "postgres://{}:{}@{}:{}/{}",
                self.username, self.password, self.host, self.port, self.database_name
            )
    } }