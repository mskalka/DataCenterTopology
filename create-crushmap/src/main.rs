extern crate juju;

use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;

/*
Here is where the controller takes input from the subordinate services,
determines which nodes are in the same failure domain, and finally
creates the crushmap from those clusters.

*/


fn main (){

    let relations: Vec<Relation> = juju::relation_list().unwrap();
    let units: HashMap<String, Vec<Ipv4Addr>> = HashMap::new();

    for unit in relations {

        let relationdata: String = juju::relation_get_by_unit(&"neighbors".to_string(), unit);


    }


    let config: HashMap<String, String> = juju::config_get_all().unwrap();


}