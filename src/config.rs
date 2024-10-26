use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub port: u16,
}

impl Config {
    pub fn init() -> Config {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

        let jwt_secret = env::var("JWT_SECRET").unwrap_or("XXX".to_string());

        let jwt_maxage = env::var("JWT_SECRET")
            .unwrap_or("3000".to_string())
            .parse::<i64>()
            .unwrap();

        let port = env::var("PORT")
            .unwrap_or("8080".to_string())
            .parse::<u16>()
            .unwrap();
        Config {
            database_url,
            redis_url,
            jwt_secret,
            jwt_maxage,
            port,
        }
    }
}
