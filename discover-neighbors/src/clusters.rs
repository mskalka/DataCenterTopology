use std::net::Ipv4Addr;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Machine {
    pub ipaddr: Ipv4Addr,
    pub neighbors: Vec<String>
}

#[derive(Debug, Clone)]
pub struct Cluster {
    pub cluster_id: i16,
    pub machines: Vec<Machine>
}

