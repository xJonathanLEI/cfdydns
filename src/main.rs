use std::{
    thread::sleep,
    time::{Duration, SystemTime},
};

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use log::{debug, error, info, trace};

mod cloudflare;
use cloudflare::CloudflareClient;

const COMMENT: &str = "Maintained by cfdydns";

#[derive(Debug, Parser)]
#[clap(about)]
struct Cli {
    /// Fully-qualified domain name to set A record on.
    #[clap(long, env = "CFDYDNS_FQDN")]
    fqdn: String,
    /// Zone name of the FQDN (e.g. `example.com`).
    #[clap(long, env = "CFDYDNS_ZONE")]
    zone: String,
    /// Cloudflare API token with the `DNS: Edit` permission for the target zone.
    #[clap(long, env = "CFDYDNS_API_TOKEN")]
    api_token: String,
    /// Number of seconds to wait between each check.
    #[clap(long, env = "CFDYDNS_INTERVAL", default_value = "300")]
    interval: u64,
}

fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "cfdydns=debug");
    }

    env_logger::init();

    let cli = Cli::parse();

    let client = CloudflareClient::new(cli.api_token);

    trace!("Looking up zone ID for {}", cli.zone);
    let zone_id = client.get_zone_id(&cli.zone)?;
    debug!(
        "Zone ID for {}: {}",
        cli.zone,
        zone_id.to_string().bright_yellow()
    );

    let interval = Duration::from_secs(cli.interval);

    loop {
        let start_time = SystemTime::now();

        if let Err(err) = run_once(&client, &zone_id, &cli.fqdn) {
            error!("Error running bot: {}", err);
        }

        let end_time = SystemTime::now();

        let run_time = end_time.duration_since(start_time)?;
        if run_time < interval {
            let wait_time = interval - run_time;
            trace!(
                "Sleeping for {} seconds before next check",
                wait_time.as_secs_f32()
            );

            sleep(wait_time);
        }
    }
}

fn run_once(client: &CloudflareClient, zone_id: &str, fqdn: &str) -> Result<()> {
    trace!("Looking up public IPv4 address");
    let public_ip = client.get_public_ip_address()?;
    debug!("Public IPv4 address: {}", public_ip.bright_yellow());

    let a_record = client.get_a_record(zone_id, fqdn)?;
    if a_record.content == public_ip && a_record.comment == COMMENT {
        debug!("Cloudflare A record is up to date");
    } else {
        trace!("Updating Cloudflare A record");
        client.update_a_record(zone_id, &a_record.id, &public_ip, COMMENT)?;
        info!(
            "Successfully updated Cloudflare A record to: {}",
            public_ip.bright_yellow()
        );
    }

    Ok(())
}
