extern crate serde_json;
extern crate serde;
extern crate hyper;
extern crate regex;

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));


mod downloader;
mod deser;
use std::thread;
use std::time::Duration;




fn main() {


    let mut de = deser::JsonSiteDeser::new();
    de.start();
    loop {
        thread::park_timeout(Duration::from_secs(10));
        println!("main --> Buffer_length: {}",de.get_buff_len());
    }
/*
    let mut dl = downloader::Downloader::new();
    dl.start();

    let mut str = String::new();
    loop {
        loop {
            thread::sleep(Duration::from_millis(1000));
            let s = dl.get_json_string();
            match s {
                Some(x) => {
                    str = x;
                    break
                },
                None => println!("waiting"),
            }
        }
        let now = Instant::now();
        let json: JsonSite = de::from_str(str.as_str()).unwrap();
        let dur = now.elapsed();
        println!("deserialized in: {}.{}s",dur.as_secs(),dur.subsec_nanos());
    }
*/

}
