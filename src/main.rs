extern crate serde_json;
extern crate serde;
extern crate hyper;
extern crate regex;

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));


mod downloader;
mod parser;
mod deser;
use regex::Regex;
use std::thread;
use std::time::Duration;




fn main() {

    let mut dw = downloader::Provider::new();
    dw.start();
    let mut de = deser::JsonSiteDeser::new();
    de.start(dw);
    let re: Regex = Regex::new("^\\+{0,1}[0-9]{1,3}%{0,1}").unwrap();
    loop {
        thread::park_timeout(Duration::from_secs(10));
        let l = de.get_buff_len();
        println!("main --> Buffer_length: {}",l);
        if(l > 0) {
            let s = de.get_next_jsonsite().unwrap();
            for s in s.stashes.iter() {
                for i in s.items.iter(){
                   match i.explicit_mods{
                       Some(ref x) => {
                           for m in x.iter(){
                                if re.is_match(m.as_str()){
                                    print!("{} ",m)
                                } else {
                                    print!("------------------ ")
                                }

                           }
                           println!("");
                       },
                       None => continue,
                    }
                }
            }
        }


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
