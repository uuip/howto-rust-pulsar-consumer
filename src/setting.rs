use deadpool_postgres::{Config, Pool, Runtime};
use reqwest::Url;
use tokio_postgres::NoTls;

#[derive(Debug)]
pub struct Setting {
    pub pulsar_addr: String,
    pub topic: String,
    pub sub_name: String,
    pub rpc: String,
    pub batch_size: i32,
    pub token_a: String,
    pub token_b: String,
    pub token_c: String,
    pub token_d: String,
    pub token_e: String,
}

pub fn get_str_env(key: &str) -> String {
    dotenvy::var(key).unwrap_or_else(|_| panic!("lost {key}"))
}

impl Setting {
    pub fn init() -> Self {
        let pulsar_addr = get_str_env("PULSAR_URL");
        let topic = get_str_env("PULSAR_TOPIC");
        let sub_name = get_str_env("PULSAR_SUB_NAME");
        let rpc = get_str_env("RPC");
        let batch_size = dotenvy::var("BATCH_SIZE")
            .expect("lost BATCH_SIZE")
            .parse::<i32>()
            .unwrap_or(40);
        let token_a = get_str_env("TOKEN_A");
        let token_b = get_str_env("TOKEN_B");
        let token_c = get_str_env("TOKEN_C");
        let token_d = get_str_env("TOKEN_D");
        let token_e = get_str_env("TOKEN_E");
        Self {
            pulsar_addr,
            topic,
            sub_name,
            rpc,
            batch_size,
            token_a,
            token_b,
            token_c,
            token_d,
            token_e,
        }
    }
}

pub async fn connection() -> Pool {
    let db_url = dotenvy::var("DB_URL").unwrap_or_else(|_| panic!("lost DB_URL"));
    let max_size = dotenvy::var("BATCH_SIZE")
        .unwrap_or_else(|_| panic!("lost BATCH_SIZE"))
        .parse::<usize>()
        .unwrap_or(50);
    let dsn = Url::parse(&db_url).unwrap();
    let mut cfg = Config::new();
    cfg.host = dsn.host().map(|x| x.to_string());
    cfg.port = dsn.port();
    cfg.dbname = dsn.path_segments().unwrap().next().map(|x| x.to_string());
    cfg.user = Some(dsn.username().to_string());
    cfg.password = dsn.password().map(|x| x.to_string());
    cfg.options = Some("-c LC_MESSAGES=en_US.UTF-8".into());
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    pool.resize(max_size);
    pool
}
