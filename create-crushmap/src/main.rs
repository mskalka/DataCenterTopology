extern crate juju;

use std::collections::{HashMap, HashSet};
use std::env;

/*
Here is where the controller takes input from the subordinate services,
determines which nodes are in the same failure domain, and finally
creates the crushmap from those clusters.

*/


fn main (){

    let juju_relation_ids = juju::relation_ids_by_identifier("controller").unwrap();
    let relation_id = &juju_relation_ids[0];
    let controller_id = env::var("JUJU_UNIT_NAME").unwrap_or("".to_string());

    let controller = parse_unit_into_relation(controller_id);

    let juju_related_units = juju::relation_get_by_id("related-units", &relation_id, &controller).unwrap();
    let mut juju_parsed_units: Vec<juju::Relation> = vec![];

    for unit in juju_related_units.split_whitespace() {
        juju_parsed_units.push(parse_unit_into_relation(unit.to_string()));
    }

    let mut machines: HashMap<String, Vec<String>> = HashMap::new();

    for unit in juju_parsed_units {
        let hostname = juju::relation_get_by_id("hostname", &relation_id, &unit).unwrap();
        let neighbors_raw = juju::relation_get_by_id("neighbors", &relation_id, &unit).unwrap();
        let hostname_trimmed = hostname.trim_matches('\n').trim();
        let neighbors_trimmed = neighbors_raw.trim_matches('\n').trim();

        let neighbors: Vec<String> = neighbors_trimmed.split_whitespace()
                                                    .map(|item| item.to_owned())
                                                    .collect();

        println!("Hostname:{}, Neighbors:{:?}", hostname_trimmed, neighbors_trimmed);
        machines.insert(hostname_trimmed.to_owned(), neighbors);
    }

    let mut racks: HashMap<usize, HashSet<String>> = HashMap::new();
    let mut potential_racks: Vec<HashSet<String>> = vec![];
    let mut rack_id: usize = 0;

    for (machine, neighbors) in machines {
        let mut members = HashSet::new();
        members.insert(machine);
        for neighbor in neighbors {
            members.insert(neighbor.clone());
        }
        potential_racks.push(members);
    }
    println!("Potential racks: {:?}", potential_racks);

    racks.insert(rack_id, potential_racks[0].clone());
    rack_id += 1;
    let new_racks = racks.clone();
    for rack in potential_racks.iter() {
        for (rack_key, existing_rack) in new_racks.iter() {
            if rack == existing_rack {
                break;
            }
            racks.insert(rack_id, rack.clone());
            rack_id += 1;
        }
    }

    println!("Racks: {:?}", racks);

}



fn parse_unit_into_relation(unit: String) -> juju::Relation {
    let v: Vec<&str> = unit.split('/').collect();
    let id: usize = v[1].parse::<usize>().unwrap();
    let parsed_unit = juju::Relation {
        name: v[0].to_string(),
        id: id,
    };
    parsed_unit
}

