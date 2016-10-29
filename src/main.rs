extern crate serde_json;
extern crate serde;
extern crate hyper;
extern crate regex;

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));


mod downloader;
mod parser;
mod deser;
use std::time::{Instant, Duration};
use regex::Regex;
use std::thread;





fn main() {

    let mut dw = downloader::Provider::new();
    dw.start();
    let mut de = deser::JsonSiteDeser::new();
    de.start(dw);
    let reg = vec![Regex::new("^\\+?([0-9]{1,3})?%?.*").unwrap()];
    let par = parser::Parser::new( reg, Regex::new("[0-9]+").unwrap(),Regex::new("([0-9]+)[.-]?([0-9]+)?").unwrap());
    let mut sinum = 0;
    loop {
        thread::park_timeout(Duration::from_secs(1));
        let l = de.get_buff_len();
        println!("main --> Buffer_length: {}",l);


        if l > 0 {
            let now = Instant::now();
            let s = de.get_next_jsonsite().unwrap();
            let mut snum = 0;
            let mut errors: usize = 0;
            for stash in s.stashes {
                let mut inum = 0;
                for item in stash.items {
                    match par.parse_item(item, &stash.stash_id) {
                        Ok(x) => {                    println!("{:?}",x);
                        },//println!("item parsed succesfully Itemnumber: {} Stashnumber: {} Sitenumber: {}", inum,snum,sinum),
                        Err(x) => {
                            println!("Error: {} Itemnumber: {} Stashnumber: {} Sitenumber: {}", x, inum, snum,sinum);
                            errors += 1;
                        },
                    }
                    inum += 1;
                }
                snum += 1;
            }
            let elapsed = now.elapsed();
            sinum +=1;
            println!("Parsed Site in {}.{}",elapsed.as_secs(),elapsed.subsec_nanos());

            /*for s in s.stashes.iter() {
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
            }*/

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
