extern crate pnet;
extern crate juju;

use std::net::Ipv4Addr;
use std::env;
use std::str;
use std::u8;

mod clusters;
mod networking;

fn main() {

    let juju_unit_ips_raw: String = juju::relation_get("unitips").unwrap();

    let mut juju_unit_ips: Vec<Ipv4Addr> = vec![];

    for ip in juju_unit_ips_raw.split_whitespace() {
        let octets: Vec<&str> = ip.split(".").collect();
        juju_unit_ips.push(Ipv4Addr::new(u8::from_str_radix(octets[0], 10).unwrap(),
                                         u8::from_str_radix(octets[1], 10).unwrap(),
                                         u8::from_str_radix(octets[2], 10).unwrap(),
                                         u8::from_str_radix(octets[3], 10).unwrap()));
    }
    //Get list of neighbor IPs using arping
    let mut neighbor_ips = networking::send_and_receive(juju_unit_ips);

    // Clean up list and remove any IPs that are not in the nodeIPlist given by juju
    neighbor_ips.sort();
    neighbor_ips.dedup();

    let mut neighbor_list: String = "".to_string();
    for address in neighbor_ips {
        let add = format!("{}", address);
        let add = add.to_string();
        neighbor_list = neighbor_list + &add + " ";

    }
    let uuid = juju::relation_get("uuid");
    juju::relation_set("neighbors", &neighbor_list);
}