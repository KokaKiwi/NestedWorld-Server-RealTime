use std::net::SocketAddr;
use std::path::Path;

config_struct! {
    pub struct Config {
        pub database: DatabaseConfig = DatabaseConfig::default(),
        pub server: ServerConfig = ServerConfig::default(),
    }
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Config {
        use toml_config::ConfigFactory;
        ConfigFactory::load(path.as_ref())
    }
}

config_struct! {
    pub struct DatabaseConfig {
        pub url: String = "postgres://nestedworld@localhost/nestedworld".to_owned(),
    }
}

config_struct! {
    pub struct ServerConfig {
        pub listen: String = "127.0.0.1:6464".to_owned(),
    }
}

impl ServerConfig {
    pub fn listen(&self) -> SocketAddr {
        self.listen.parse().unwrap()
    }
}
