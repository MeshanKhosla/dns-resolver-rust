#[derive(Debug)]
struct RootServer {
    hostname: &'static str,
    ipv4: &'static str
}

// https://www.iana.org/domains/root/servers
const ROOT_SERVERS: [RootServer; 1] = [
    RootServer {
        hostname: "a.root-servers.net",
        ipv4: "198.41.0.4"
    }
];

pub fn resolve() {
    println!("{}", ROOT_SERVERS[0].hostname);
    println!("{}", ROOT_SERVERS[0].ipv4);
}