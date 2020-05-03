extern crate snmp_agent;
// extern crate clap;

use clap::Clap;
  
use std::time::Instant;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = VERSION, author = "Andy P. <andy@failfree.net>")]
struct Opts {
    /// Sets a custom port, defaults to 1161
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