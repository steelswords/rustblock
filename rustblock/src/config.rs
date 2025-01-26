use serde::Deserialize;
use toml;
use std::collections::HashMap;


//#[derive(Deserialize, Debug)]
//pub struct TimeInterval {
//    // TODO: Make this more robust
//    minutes: Option<i32>,
//}

#[derive(Deserialize, Debug)]
pub struct IntermittentBlockDefinition {
    pub host: String,
    //pub length: TimeInterval,
    pub minutes_on: u32,
    // TODO later: pub minutes_off: i32,
    pub day_total: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct BlackoutTime {
    name: Option<String>,
    start: String,
    end: String,
    exceptions: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct BlockProfile {
    pub macs: Vec<String>,
    pub always_block: Vec<String>,
    pub intermittent_block: Vec<IntermittentBlockDefinition>,
}

#[derive(Deserialize, Debug)]
pub struct ConfigOptions {
    pub blockprofiles: HashMap<String, BlockProfile>,
    pub websites: HashMap<String, WebsiteDefinition>,
}

#[derive(Deserialize, Debug)]
pub struct WebsiteDefinition {
    pub domains: Vec<String>,
}



#[cfg(target_arch = "x86_64")]
pub fn get_config_file_name() -> String {
    "./blockprofiles.toml".to_string()
}

#[cfg(target_arch = "arm")]
pub fn get_config_file_name() -> String {
    "/jffs/blockprofiles.toml".to_string()
}
