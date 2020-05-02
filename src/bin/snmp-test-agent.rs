extern crate snmp_agent;

use std::time::Instant;

fn main() {
    println!("SNMP Test Agent started");
    let start_time = Instant::now();

    snmp_agent::run();

    println!("SNMP Test Agent completed in {}ms", start_time.elapsed().as_millis())
}