#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
#[macro_use] extern crate lazy_static;

extern crate serde_json;
extern crate serde;
extern crate hyper;
extern crate regex;
extern crate time;



mod serde_types;
mod downloader;
mod parser;
mod deser;
use std::sync::mpsc;
use std::time::{Instant, Duration};
use regex::Regex;
use std::thread;





fn main() {

    let mut provider = downloader::Provider::new();

    let (send_dw_to_deser, recv_deser_from_dw) = mpsc::channel();
    let (send_deser_to_parser, recv_parser_from_deser) = mpsc::channel();
    let (send_todb, _) = mpsc::channel();

    provider.start(send_dw_to_deser);
    let mut de = deser::JsonSiteDeser::new();
    de.start(provider, recv_deser_from_dw, send_deser_to_parser);
    let mut par = parser::Parser::new(send_todb,recv_parser_from_deser,de);
    thread::spawn(move|| {
        par.start_parsing();
    });
    loop {
        thread::park_timeout(Duration::from_secs(1));
    }

}
