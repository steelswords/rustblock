
mod config;

use std::fs::read_to_string;

use config::WebsiteAddressTable;

// TODO: All of this
fn main() {
    let websites_toml_contents = read_to_string("websites.toml").unwrap();
    let websites_addresses : WebsiteAddressTable = toml::from_str(&websites_toml_contents).unwrap();

    for (website, table_entry) in websites_addresses.websites.iter()
    {
        println!("Website '{}' has addresses of {:?}", &website, &table_entry.addresses);
    }
}
