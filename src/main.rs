#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
#[macro_use] extern crate lazy_static;

extern crate postgres;
extern crate serde_json;
extern crate serde;
extern crate hyper;
extern crate regex;
extern crate time;

mod logger;
mod pgsql;
mod serde_types;
mod downloader;
mod parser;
mod deser;
use logger::Logger;
use std::sync::mpsc;
use std::thread;





fn main() {

    let (to_logger, logger_recv) = mpsc::channel();
    let (send_dw_to_deser, recv_deser_from_dw) = mpsc::channel();
    let (send_deser_to_parser, recv_parser_from_deser) = mpsc::channel();
    let (send_todb, _) = mpsc::channel();
    let mut downloader = downloader::Downloader::new(send_dw_to_deser,to_logger.clone());

    thread::spawn( move || {
        downloader.init( String::from("7222850-7781124-7106063-8472575-7898327"));
    });

    let mut deser = deser::PoeDeser::new(send_deser_to_parser,recv_deser_from_dw,to_logger.clone());

    thread::spawn( move || {
        deser.init();
    });

    let mut par = parser::Parser::new(send_todb, recv_parser_from_deser, to_logger.clone());
    thread::spawn(move|| {
        par.start_parsing();
    });
    let logger = Logger::new(logger_recv);
    thread::spawn(move|| {
        logger.init();
    });

    loop {
        thread::park();
    }

}
