extern crate serde_json;
extern crate serde;
extern crate hyper;

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

use serde_json::de;
use hyper::client::Client;
use std::io::prelude::{Read};
use std::str;
use std::fmt::Write;




fn main() {
    let client = Client::new();
    println!("client started");
    let mut res = client.get("http://api.pathofexile.com/public-stash-tabs").send().unwrap();
    println!("request done! ");
    let mut json: JsonSite = de::from_reader(res).unwrap();
    println!("deserialized");
    println!("next_id: {}",json.next_change_id);

   /* let mut get: String = String::new();
    loop {
        get.truncate(0);
        write!(&mut get,"http://api.pathofexile.com/public-stash-tabs?id={}",id);
        println!("get {}",get.as_str());
        res = client.get(get.as_str()).send().unwrap();
        println!("request done! ");
        json = de::from_reader(res).unwrap();
        println!("deserialized");
        println!("next_id: {}",json.next_change_id);
        id = json.next_change_id;
    }*/


}
