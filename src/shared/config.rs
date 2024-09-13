use std::{env::var, error::Error};

pub struct Environment {
    pub port: u16,
    pub db_username: String,
    pub db_host: String,
    pub db_password: String,
    pub db_port: u16,
    pub db_entry: String,
}

impl Environment {
    pub fn config() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            port: var("PORT")?.parse()?,
            db_username: var("DB_USER")?,
            db_host: var("DB_HOST")?,
            db_password: var("DB_PASSWORD")?,
            db_port: var("DB_PORT")?.parse()?,
            db_entry: var("DB_ENTRY")?,
        })
    }
}
