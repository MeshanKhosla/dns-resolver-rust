mod root_nameserver_resolver;
use clap::{Parser, ValueEnum};

// https://www.cloudflare.com/learning/dns/dns-records/
#[derive(Debug, Clone, ValueEnum)]
#[value(rename_all = "verbatim")] // Make this case sensitive
enum RecordType {
    A,
    AAAA,
    CNAME,
    MX,
    TXT,
    NS,
}

#[derive(Debug, Parser)]
pub struct Resolution {
    /// Domain to look up (e.g. github.com)
    domain: String,

    /// The record type being requested
    record_type: RecordType,
}

// Invoked like `my-dns github.com A`
fn main() {
    let resolution = Resolution::parse();
    let domain = resolution.domain;
    let record_type = resolution.record_type;
    println!("Looking for Domain: {} with record {:?}...", domain, record_type);

    root_nameserver_resolver::resolve();
}
