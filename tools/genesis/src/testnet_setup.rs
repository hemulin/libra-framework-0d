use anyhow::{Context};
use clap::{Args, Subcommand};
use libra_config::host;
use crate::{
    genesis_builder, parse_json,
    supply::SupplySettings,
    wizard::{GenesisWizard, GITHUB_TOKEN_FILENAME},
};
use libra_types::{exports::NamedChain, global_config_dir, legacy_types::fixtures::TestPersona};
use std::{path::PathBuf, net::Ipv4Addr, fs, time, thread};
use zapatos_genesis::config::{HostAndPort, ValidatorConfiguration};


pub fn setup(me: &TestPersona, ip_list: &[Ipv4Addr], chain: NamedChain, data_path: PathBuf) -> anyhow::Result<()>{;

            let db_path = data_path.join("data");
            println!("WARN: deleting {}, in 5 secs", db_path.display());
            let delay = time::Duration::from_secs(5);
            thread::sleep(delay);
            fs::remove_dir_all(db_path)?;

            // create the local files for my_persona
            let index = me.idx();
            let format_host_str = format!("{}:6180", ip_list.get(index).expect("could not get an IP and index").to_string());
            println!("your persona {me:?}is expected to use IP: {}", format_host_str);
            let my_host: HostAndPort = format_host_str.parse().expect("could not parse IP address for host");
            host::initialize_host(
              Some(data_path.clone()),
              Some(&me.to_string()),
              my_host,
              Some(me.get_persona_mnem()),
              false,
            )?;

            // create validator configurations from fixtures
            // without needing to use a github repo to register and read
            let val_cfg: Vec<ValidatorConfiguration> = ip_list
                .iter()
                .enumerate()
                .filter_map(|(idx, ip)| {

                    let format_host_str = format!("{}:6180", ip.to_string());
                    let host: HostAndPort = format_host_str.parse().expect("could not parse IP address for host");
                    let p = TestPersona::from(idx).ok()?;
                    genesis_builder::testnet_validator_config(&p, &host).ok()
                })
                .collect();

            let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/sample_export_recovery.json");

            let recovery = parse_json::recovery_file_parse(p)?;

            genesis_builder::build(
                "none".to_string(), // when is testnet is ignored
                "none".to_string(),
                "none".to_string(),
                data_path,
                true,
                Some(&recovery),
                Some(SupplySettings::default()),
                chain,
                Some(val_cfg),
            )?;
        Ok(())
}