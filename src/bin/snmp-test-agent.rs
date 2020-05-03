extern crate snmp_agent;
// extern crate clap;

use clap::Clap;
  
use std::time::Instant;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// This program acts as a self contained SNMP agent on localhost.
/// It generates dummy data which is suitable for testing an SNMP Manager.
/// It uses the snmp-agent library
#[derive(Clap)]
#[clap(version = VERSION, author = "Andy P. <andy@failfree.net>")]
struct Opts {
    /// Sets the port to listen on
    #[clap(short = "p", long = "port", default_value = "1161")]
    port: u16,
}

fn main() {
    let opts: Opts = Opts::parse();

    println!("SNMP Test Agent started");
    let start_time = Instant::now();

    snmp_agent::run(opts.port);

    println!("SNMP Test Agent completed in {}ms", start_time.elapsed().as_millis())
}