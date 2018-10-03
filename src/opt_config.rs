use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Parker")]
pub struct Opt {
    /// Config file path
    #[structopt(short = "c", long = "config", default_value = "parker.toml")]
    pub config_file: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub database_uri: String,
    pub database_name: String,
    pub static_dir: String,
}

impl Config {
    pub fn load(cfg_file: &str) -> Config {
        let mut cfg_handler = config::Config::default();
        cfg_handler
            .merge(config::File::with_name(&cfg_file))
            .unwrap();
        cfg_handler.try_into().unwrap()
    }
}
