extern crate pnet;
extern crate juju;
extern crate log;

use std::str::FromStr;
use std::net::Ipv4Addr;
use std::env;
use std::str;
use std::process::Command;
use std::collections::HashMap;
use log::LogLevel;

mod networking;

fn main() {

    let juju_relation_ids = juju::relation_ids_by_identifier("controller").unwrap();
    println!("Relation ids: {:?}", juju_relation_ids);

    let relation_id = &juju_relation_ids[0];

    let controller = juju::relation_list_by_id(&relation_id).unwrap();

    let juju_unit_list: String = juju::relation_get_by_id("related-units", &relation_id, &controller[0]).unwrap();
    println!("Unit list: {}", juju_unit_list);

    let mut juju_machine_ids_with_ip: HashMap<String, Ipv4Addr> = HashMap::new();

    for unit in juju_unit_list.split_whitespace() {

        println!("Unit to decompose: {}", unit);

        let identifier: Vec<&str> = unit.split('/').collect();
        let name:String = identifier[0].to_owned();
        let id = identifier[1].parse::<usize>().unwrap();
        let relation = juju::Relation {name: name, id: id};
        let ip = juju::relation_get_by_id("private-address", &relation_id, &relation).unwrap();
        let hostname = juju::relation_get_by_id("hostname", &relation_id, &relation).unwrap();
        let ip = ip.trim();
        println!("{}", &ip);
        juju_machine_ids_with_ip.insert(hostname, Ipv4Addr::from_str(&ip).unwrap());
    }
    println!("Known IPs: {:?}", juju_machine_ids_with_ip);

    //Get list of neighbor IPs using arping
    let neighbor_list = networking::send_and_receive(juju_machine_ids_with_ip);

    let neighbors_formatted = format!("{:?}",neighbor_list);

    juju::relation_set_by_id("neighbors", &neighbors_formatted, &relation_id);
}