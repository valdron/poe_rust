extern crate serde_json;
extern crate serde;
extern crate hyper;
extern crate regex;

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

use regex::Regex;
use serde_json::de;
use hyper::client::Client;
use std::io::prelude::{Read};
use std::str::FromStr;
use std::fmt::Write;
use std::thread;




fn main() {
    let client = Client::new();
    println!("client started");
    let mut res = client.get("http://api.pathofexile.com/public-stash-tabs").send().unwrap();
    println!("request done! ");


    let mut buff: [u8; 45] = [0; 45];
    let _ = res.read_exact(&mut buff);
    println!("{}", std::str::from_utf8(&buff).unwrap());

    let re = Regex::new("[0-9]*-[0-9]*-[0-9]*-[0-9]*-[0-9]*").unwrap();
    let id = re.captures_iter(std::str::from_utf8(&buff).unwrap()).next().unwrap().at(0).unwrap();
    println!("{}", id);

    let mut json_buff = Vec::from(&buff[..]);
    let _ = res.read_to_end(&mut json_buff);

    let mut json: JsonSite = de::from_slice(json_buff.as_slice()).unwrap();
    println!("deserialized");
    println!("next_id: {}", json.next_change_id);
    /*
   let mut get: String = String::new();
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
    }
*/

}
