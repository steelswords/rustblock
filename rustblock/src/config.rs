use serde::Deserialize;
use std::collections::HashMap;
use iptables;


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
    pub user_device_hosts: Vec<String>,
    pub always_block: Vec<String>,
    pub intermittent_block: Vec<IntermittentBlockDefinition>,
    pub blackout_times: Vec<BlackoutTime>,
}

#[derive(Deserialize, Debug)]
pub struct ConfigOptions {
    pub blockprofiles: HashMap<String, BlockProfile>,

    #[serde(default = "get_default_websites_toml_path")]
    pub websites_toml: String,

    #[serde(skip)]
    pub websites: HashMap<String, WebsiteAddressTableEntry>,
}

fn get_default_websites_toml_path() -> String {
    String::from("websites.toml")
}

#[derive(Deserialize, Debug)]
pub struct WebsiteAddressTable {
    pub websites: HashMap<String, WebsiteAddressTableEntry>,
}

#[derive(Deserialize, Debug)]
pub struct WebsiteAddressTableEntry {
    pub addresses: Vec<String>
}


#[cfg(target_arch = "x86_64")]
pub fn get_config_file_name() -> String {
    "./blockprofiles.toml".to_string()
}

#[cfg(target_arch = "arm")]
pub fn get_config_file_name() -> String {
    "/jffs/blockprofiles.toml".to_string()
}
