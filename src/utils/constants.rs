use std::env;
use std::sync::OnceLock;

pub fn get_address() -> &'static String {
    static ADDRESS: OnceLock<String> = OnceLock::new();
    ADDRESS.get_or_init(|| env::var("ADDRESS").unwrap())
}

pub fn get_port() -> &'static u16 {
    static PORT: OnceLock<u16> = OnceLock::new();
    PORT.get_or_init(|| env::var("PORT").unwrap().parse::<u16>().unwrap())
}

pub fn db_url() -> &'static String {
    static DATABASE_URL: OnceLock<String> = OnceLock::new();
    DATABASE_URL.get_or_init(|| env::var("DATABASE_URL").unwrap())
}

pub fn get_secret() -> &'static String {
    static SECRET: OnceLock<String> = OnceLock::new();
    SECRET.get_or_init(|| env::var("SECRET").unwrap())
}
