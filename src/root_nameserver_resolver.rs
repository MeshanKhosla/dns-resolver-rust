use crate::{Resolution, dns::build_dns_message, dns_wireformat::WireFormat};
use rand::seq::IndexedRandom;
use std::{io::Error, net::UdpSocket, time::Duration};

#[derive(Debug)]
struct RootServer {
    hostname: &'static str,
    ipv4: &'static str,
}

// https://www.iana.org/domains/root/servers
const ROOT_SERVERS: [RootServer; 13] = [
    RootServer {
        hostname: "a.root-servers.net",
        ipv4: "198.41.0.4",
    },
    RootServer {
        hostname: "b.root-servers.net",
        ipv4: "170.247.170.2",
    },
    RootServer {
        hostname: "c.root-servers.net",
        ipv4: "192.33.4.12",
    },
    RootServer {
        hostname: "d.root-servers.net",
        ipv4: "199.7.91.13",
    },
    RootServer {
        hostname: "e.root-servers.net",
        ipv4: "192.203.230.10",
    },
    RootServer {
        hostname: "f.root-servers.net",
        ipv4: "192.5.5.241",
    },
    RootServer {
        hostname: "g.root-servers.net",
        ipv4: "192.112.36.4",
    },
    RootServer {
        hostname: "h.root-servers.net",
        ipv4: "198.97.190.53",
    },
    RootServer {
        hostname: "i.root-servers.net",
        ipv4: "192.36.148.17",
    },
    RootServer {
        hostname: "j.root-servers.net",
        ipv4: "192.58.128.30",
    },
    RootServer {
        hostname: "k.root-servers.net",
        ipv4: "193.0.14.129",
    },
    RootServer {
        hostname: "l.root-servers.net",
        ipv4: "199.7.83.42",
    },
    RootServer {
        hostname: "m.root-servers.net",
        ipv4: "202.12.27.33",
    },
];

impl RootServer {
    fn resolve(&self, resolution: &Resolution) -> Result<&str, Error> {
        // DNS is on port 53
        let server_ip = format!("{}:53", self.ipv4);
        // Send from random local port
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_read_timeout(Some(Duration::from_secs(5)))?;
        let dns_message = build_dns_message(&resolution);
        let result = socket.send_to(dns_message.encode().as_slice(), &server_ip);
        // TODO: recv
        dbg!(result);
        // TODO: This is probably wrong, it will likely be a DNS Message/RR
        Ok("Future IP")
    }
}

// TODO: Return Result type and get rid of expects
pub fn resolve(resolution: &Resolution) {
    println!(
        "[Root nameserver resolution] Querying for domain: {} with record {:?}",
        resolution.domain, resolution.record_type
    );
    let random_root_server = ROOT_SERVERS
        .choose(&mut rand::rng())
        .expect("Root server should exist");
    random_root_server.resolve(resolution);
}
