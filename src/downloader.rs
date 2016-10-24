extern crate hyper;
extern crate regex;

use regex::Regex;
use std::thread::*;
use hyper::client::{Response, Client};
use std::sync::{Arc, Mutex};
use std::str;
use std::io::prelude::*;


pub struct Downloader {
    responses: Arc<Mutex<Vec<String>>>,
    client: Client,
    re: Regex,
    next_id: String

}

impl Downloader {
    pub fn new() -> Downloader{
        Downloader{
            next_id: String::new(),
            re: Regex::new("[0-9]*-[0-9]*-[0-9]*-[0-9]*-[0-9]*").unwrap(),
            responses: Arc::new(Mutex::new(Vec::new())),
            client: Client::new()
        }
    }

    pub fn start(&mut self) {
        let mut thr_self = Downloader{
            responses: self.responses.clone(),
            client: Client::new(),
            re: Regex::new("[0-9]*-[0-9]*-[0-9]*-[0-9]*-[0-9]*").unwrap(),
            next_id: String::new(),
        };
        let thread = spawn(move || {
            loop{
                let url = thr_self.build_new_url(thr_self.next_id.as_str());
                println!("Downloader --> new url:{}",url);
                let res = thr_self.request(url.as_str());
                println!("Downloader --> request done");
                let js_str = thr_self.read_to_str(res);
                println!("Downloader --> read to string");
                let _ = thr_self.get_next_id(&js_str);
                println!("Downloader --> got next id:{}",thr_self.next_id);
                thr_self.push_string_to_vec(js_str);
                println!("Downloader --> pushed to vec");
            }

        });
    }

    fn request(&self, url: &str) -> hyper::client::Response {
        self.client.get(url).send().unwrap()
    }

    fn build_new_url(&self, id: &str) -> String{
        let mut url = String::from("http://api.pathofexile.com/public-stash-tabs");
        url.push_str("?id=");
        url.push_str(id);
        url
    }

    fn get_next_id(&mut self, s: &String) {
        self.next_id = String::from(self.re.captures(s.as_str()).unwrap().at(0).unwrap());
    }

    pub fn get_json_string(&mut self) -> Option<String> {
        let mut guard = self.responses.lock().unwrap();
        (*guard).pop()
    }


    fn read_to_str(&self, mut res: Response) -> String {
        let mut buff: Vec<u8> = Vec::new();
        let _ = res.read_to_end(&mut buff);
        String::from_utf8(buff).unwrap()

    }

    fn push_string_to_vec(&mut self, s: String) {
        let mut guard = self.responses.lock().unwrap();
        (*guard).push(s);
    }
}

