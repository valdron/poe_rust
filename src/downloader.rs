extern crate hyper;
extern crate regex;

use regex::Regex;
use std::thread;
use hyper::client::{Response, Client};
use std::sync::{Arc, Mutex};
use std::str;

struct Downloader<'a> {
    responses: Arc<Mutex<Vec<&str>>>,
    client: Client,
    re: Regex

}

impl Downloader {
    fn new() -> Downloader{
        Downloader{
            re: Regex::new("[0-9]*-[0-9]*-[0-9]*-[0-9]*-[0-9]*"),
            responses: Arc::new(Mutex::new(Vec::new())),
            client: Client::new()
        }
    }

    fn start(&self) {

    }

    fn request(&self, url: &str) -> hyper::client::Response {
        self.client.get(url).send()
    }

    fn build_new_url(&self,id: &str) -> str{

    }

    fn read_to_str(&self, res: Response) -> str {
        let buff: Vec<u8> = Vec::new();
        let _ = res.read_to_end(&mut buff);
        str::from_utf8(buff.as_slice())

    }
}
