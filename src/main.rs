extern crate serde_json;
extern crate hyper;

use serde_json::de;
use hyper::client::Client;
use std::io::prelude::*;
use std::io;
use json_structs::JsonSite;




fn main() {
    let client = Client::new();
    let res = client.get("http://api.pathofexile.com/public-stash-tabs").send().unwrap();

    let json: JsonSite = de::from_reader(res).unwrap();

}
