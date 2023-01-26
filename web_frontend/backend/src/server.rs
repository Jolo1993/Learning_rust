use rocket::config::{Config, Environment, Value};
use std::collections::HashMap;

pub fn create_server() -> rocket::Rocket {
    let mut config = Config::build(Environment::Staging)
        .address("127.0.0.1")
        .port(8000)
        .finalize()
        .unwrap();

    // Add custom configuration options
    let mut database_config = HashMap::new();
    database_config.insert("url", Value::from("127.0.0.1:5432"));
    database_config.insert("user", Value::from("myuser"));
    database_config.insert("password", Value::from("mypassword"));
    config.set_extras(database_config);

    rocket::custom(config)
}