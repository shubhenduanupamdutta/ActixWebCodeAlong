use std::env;
use std::sync::OnceLock;

pub fn get_address() -> &'static String {
    static ADDRESS: OnceLock<String> = OnceLock::new();
    ADDRESS.get_or_init(|| env::var("ADDRESS").unwrap_or("127.0.0.1".to_string()))
}

pub fn get_port() -> u16 {
    static PORT: OnceLock<u16> = OnceLock::new();
    *PORT.get_or_init(|| {
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

pub fn get_max_file_size() -> u64 {
    static MAX_FILE_SIZE: OnceLock<u64> = OnceLock::new();
    *MAX_FILE_SIZE.get_or_init(|| {
        env::var("MAX_FILE_SIZE")
            .unwrap()
            .parse::<u64>()
            .unwrap_or(10485760)
    })
}
