use std::env;
use std::sync::OnceLock;

pub fn get_address() -> &'static String {
    static ADDRESS: OnceLock<String> = OnceLock::new();
    ADDRESS.get_or_init(|| env::var("ADDRESS").unwrap_or("127.0.0.1".to_string()))
}

pub fn get_port() -> &'static u16 {
    static PORT: OnceLock<u16> = OnceLock::new();
    PORT.get_or_init(|| {
        env::var("PORT")
            .unwrap_or("8000".to_string())
            .parse::<u16>()
            .expect("Not a valid port number that can be parsed to u16.")
    })
}

pub fn db_url() -> &'static String {
    static DATABASE_URL: OnceLock<String> = OnceLock::new();
    DATABASE_URL.get_or_init(|| env::var("DATABASE_URL").expect("No database defined."))
}

pub fn get_secret() -> &'static String {
    static SECRET: OnceLock<String> = OnceLock::new();
    SECRET.get_or_init(|| env::var("SECRET").expect("A secret needs to be defined in Environment"))
}
