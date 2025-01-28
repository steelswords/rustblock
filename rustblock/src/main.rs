use std::{fs::read_to_string, net::{IpAddr, Ipv4Addr}};
use std::str::FromStr;
use config::{BlockProfile, ConfigOptions, WebsiteAddressTable};
use dns_lookup::lookup_host;
use clap::{Args, Parser, Subcommand};
mod config;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[command(subcommand)]
    command: CliCommands,
}

#[derive(Subcommand)]
enum CliCommands {
    Enable(CliCommandsProfileArgs),

    Disable(CliCommandsProfileArgs),

    List { },
}

#[derive(Args)]
struct CliCommandsProfileArgs {
    profile_name: String,
}


fn read_config() -> ConfigOptions {
    let config_file_string = read_to_string(config::get_config_file_name())
        .expect("Could not open config file at /jffs/blockprofiles.toml. Exiting.");
    let mut config_options: config::ConfigOptions = toml::from_str(&config_file_string)
        .expect("Error in config file.");

    let websites_address_table: WebsiteAddressTable = toml::from_str(
        &read_to_string(&config_options.websites_toml).unwrap()
    ).unwrap();

    config_options.websites = websites_address_table.websites;
    config_options
}

fn print_profile(name: &String, profile: &BlockProfile, config_options: &ConfigOptions) {
        let rules_list = get_rules_from_profile(profile, config_options);

        println!("-----------------------------------------\n");
        println!(" Profile {}\n", name);
        rules_list.iter().for_each(|(src, dst)| println!("\t Block from {} -> {}", src, dst));
        println!("-----------------------------------------\n");
}

fn list_profiles(config_options: &ConfigOptions) {
    for (profile_name, profile_data) in config_options.blockprofiles.iter() {
        print_profile(profile_name, profile_data, config_options);
    }
}

fn main() -> Result<(), std::io::Error> {
    let config_options = read_config();

    let cli = CliArgs::parse();

    match cli.command {
        CliCommands::Enable(profile) => {
            if let Some(profile_name) = config_options.blockprofiles.get(&profile.profile_name) {
                enable_profile(profile_name).unwrap();
            }
            else {
                eprintln!("ERROR: Could not find profile {}", profile.profile_name);
                panic!();
            }
        },
        CliCommands::Disable(profile) => {
            println!("disable not supported yet. Can't disable profile {}", profile.profile_name);
            panic!();
        },
        CliCommands::List {} => { list_profiles(&config_options); },

        _ => panic!("Not supported"),
    }

    //for (website_name, website_aliases) in config_options.websites.iter() {
    //    println!(" Website {} has the following aliases: {:?}", website_name, website_aliases.addresses);
    //}

    Ok(())
}

// TODO: Redo
fn get_blocked_destination_ip_addresses_from_profile(profile: &config::BlockProfile, config_options: &ConfigOptions) -> Vec<String> {
    let mut result_vec = vec![];

    for blocked_site in profile.always_block.iter() {
        if let Some(blocked_domain_aliases) = config_options.websites.get(blocked_site) {
            // Since we're already forcing IP addresses now, this gets a lot shorter.
            blocked_domain_aliases.addresses
                .iter()
                .for_each(|address| result_vec.push(address.clone()));
            //for domain in blocked_domain_aliases.addresses.iter() {
            //    // Get IP address of domain, add to list
            //    match lookup_host(domain) {
            //        Ok(ip_addresses) => {
            //            // Convert all the IpAddrs to Strings and append that vec to the result_vec
            //            result_vec.append(
            //                &mut ip_addresses
            //                .iter()
            //                .map(|i| i.to_string())
            //                .collect::<Vec<String>>()
            //            );
            //        },
            //        Err(e) => eprintln!("Could not resolve IP address for {}: {}", domain, e)
            //    };
            //}
        }
        else {
            eprintln!("ERROR: Could not find a websites definition for {}", blocked_site);
        }
    }

    result_vec
}

fn get_blocked_source_ip_addresses_from_profile(profile: &config::BlockProfile) -> Vec<String> {
    let mut result_vec = vec![];
    // Gather list of user device IPs
    for user_device_host in profile.user_device_hosts.iter() {
        // If it's already an IP address, just use that.
        if let Ok(ip_address) = IpAddr::from_str(user_device_host.as_str()) {
            println!("{} was already in IpAddr format. Not resolving.", ip_address);
            result_vec.push(ip_address.to_string());
        }
        // Otherwise, resolve it to an IP address
        // TODO: There is a hole in this logic with my current dns setup where if a 
        // device is not online, it doesn't resolve to anything.... Maybe need to
        // reserve addresses or force using IP addresses.
        else {
            match lookup_host(user_device_host) {
                Ok(ip_addresses) => {

                    println!("INFO: Resolved host '{}' to IP addresses '{:?}'", &user_device_host, &ip_addresses);
                    result_vec.append(
                        &mut ip_addresses
                            .iter()
                            .map(|a| a.to_string())
                            .collect::<Vec<String>>()
                    );
                },
                Err(e) => eprintln!("ERROR: Could not resolve host {}: {}", user_device_host, e),
            };
        }
    }
    result_vec
}


/// Returns a vector of tuples. Each tuple is of the form (src, dst), where
/// src and dst define a connection that must be blocked by the firewall rules.
fn get_rules_from_profile(profile: &config::BlockProfile, config_options: &ConfigOptions) -> Vec<(String, String)> {
    // Compute list of blocked source IP addresses, i.e. the user's devices.
    let blocked_from_ips: Vec<String> = get_blocked_source_ip_addresses_from_profile(profile);

    // Compute list of blocked desitnation IP addresses
    let blocked_to_ips: Vec<String> = get_blocked_destination_ip_addresses_from_profile(profile, config_options);

    // Permute over every src-dst combo, return that vector of tuples
    let mut return_vec: Vec<(String, String)> = vec![];
    for from_ip in blocked_from_ips.iter() {
        for to_ip in blocked_to_ips.iter() {
            return_vec.push((from_ip.clone(), to_ip.clone()));
        }
    }
    return_vec
}

fn enable_profile(profile: &config::BlockProfile) -> Result<(), Box<dyn std::error::Error>> {
    println!("Enabling profile {:?}", profile);
    println!("Jk, not implemented yet.");
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
