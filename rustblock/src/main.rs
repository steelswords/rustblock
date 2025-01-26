use std::fs::read_to_string;
mod config;



fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");



    let config_file_string = read_to_string(config::get_config_file_name()).expect("Could not open config file at /jffs/blockprofiles.toml. Exiting.");
    let config_options: config::ConfigOptions = toml::from_str(&config_file_string).expect("Error in config file.");

    for (profile_name, _profile_data) in config_options.blockprofiles.iter() {
        println!("I see a profile named {}", profile_name);
    }

    for (website_name, website_aliases) in config_options.websites.iter() {
        print!(" Website {} has the following aliases: ", website_name);
        website_aliases.domains.iter().for_each(|d| print!(" {} ", d));
        println!();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_config_file() {
        let config_file_name = config::get_config_file_name();
        let config_file_string = read_to_string(&config_file_name).unwrap_or_else( |_| panic!("Could not open config file at {}. Exiting.", &config_file_name));
        let _config_options: config::ConfigOptions = toml::from_str(&config_file_string).expect("Error in config file.");

        // TODO: Could do more tests here, but not right now.
    }
}
