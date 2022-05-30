
use serde::__private::de;
use serde_derive::Deserialize;



use std::env;
use std::fs;


/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub global_string: Option<String>,
    pub TERMINAL: Option<String>,
    pub LAUNCHER: Option<String>,
}




pub fn main() -> Config { 

    
    let toml_str = fs::read_to_string("config.toml")
                                    .expect("Something went wrong reading the file");

    let decoded: Config = toml::from_str(&toml_str).unwrap();
        

     return decoded;

     let config_data = config::main();
     let terminal = config_data.TERMINAL.unwrap().to_owned();
     let launcher = config_data.LAUNCHER.unwrap().to_owned();

}