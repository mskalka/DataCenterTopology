extern crate pnet;
extern crate juju;

use std::net::Ipv4Addr;
use std::env;
use std::str;

mod clusters;
mod networking;

fn main() {

    //Get list of neighbor IPs using arping
    let mut neighbor_ips = networking::send_and_receive();

    // Clean up list and remove any IPs that are not in the nodeIPlist given by juju
    neighbor_ips.sort();
    neighbor_ips.dedup();

    let mut neighbor_list: String = "".to_string();
    for address in neighbor_ips {
        let add = format!("{}", address);
        let add = add.to_string();
        neighbor_list = neighbor_list + &add + " ";

    }
    juju::action_set(&"discover-neighbors.neighborips".to_string(), &neighbor_list);
}