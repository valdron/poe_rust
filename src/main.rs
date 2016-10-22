extern crate serde_json;
extern crate hyper;

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

use serde_json::de;
use hyper::client::Client;
use std::io::prelude::*;
use std::io;





fn main() {
    let client = Client::new();
    let res = client.get("http://api.pathofexile.com/public-stash-tabs").send().unwrap();

    let json: JsonSite = de::from_reader(res).unwrap();

}
