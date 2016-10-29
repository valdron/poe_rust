extern crate hyper;
extern crate regex;

use regex::Regex;
use std::thread::*;
use hyper::client::{Response, Client};
use std::sync::{Arc, Mutex, mpsc};
use std::str;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::time::{Instant, Duration};
use time;


pub struct Provider {
    json_strings: Arc<Mutex<VecDeque<String>>>,


}

impl Provider {
    pub fn new() -> Provider{
        Provider{
            json_strings: Arc::new(Mutex::new(VecDeque::new()))
        }
    }

    pub fn start(&mut self, sender: mpsc::Sender<String>) {
        let mut d = Downloader{
            json_strings: self.json_strings.clone(),
            responses: Arc::new(Mutex::new(VecDeque::new())),
            msg_send: None,
            msg_receive: None,
            send_to_deser: sender,
            re: Regex::new("[0-9]*-[0-9]*-[0-9]*-[0-9]*-[0-9]*").unwrap(),
        };

        let _ = spawn(move || {
            d.init();
        });
    }

    pub fn get_json_string(&mut self) -> Option<String> {
        let mut guard = self.json_strings.lock().unwrap();
        (*guard).pop_back()
    }



}

struct Downloader{
    json_strings:  Arc<Mutex<VecDeque<String>>>,
    responses: Arc<Mutex<VecDeque<hyper::client::Response>>>,
    msg_send: Option<mpsc::Sender<String>>,
    msg_receive: Option<mpsc::Receiver<String>>,
    send_to_deser: mpsc::Sender<String>,
    re: Regex,
}

impl Downloader{
    fn start_crawler(&mut self, mut c: Crawler){

        let _ = spawn(move ||{
            c.init();
        });
    }
    fn init(&mut self){
        let (send, crecv) = mpsc::channel();
        let (csend, recv) = mpsc::channel();
        self.msg_send = Some(send);
        self.msg_receive = Some(recv);

        let c = Crawler{
            responses: self.responses.clone(),
            client: Client::new(),
            msg_send_to_downloader: csend,
            recv_from_downloader: crecv,
            next_id: String::new(),
        };
        self.start_crawler(c);
        loop{
            let _ = self.msg_receive.as_mut().unwrap().recv();
            //println!("Downloader --> got msg from Crawler");
            let mut res = self.get_next_response().unwrap();
            //println!("Downloader --> got next response");
            let mut start_string: String = self.get_start(&mut res);
            let next_id = self.get_next_id(&start_string);
            //println!("Downloader --> got next id:{}",next_id);
            self.send_next_id(next_id);
            //println!("Downloader --> sent next id to crawler");
            let now = Instant::now();
            self.read_rest_to_str(res, &mut start_string);
            self.push_string_to_vec(start_string);
            let _ = self.send_to_deser.send(String::from("pushed"));
            println!("{} Downloader --> read to string and pushed in {},{}s",time::at(time::get_time()).ctime(), now.elapsed().as_secs(),now.elapsed().subsec_nanos());

        }

    }

    fn read_rest_to_str(&self, mut res: Response,  s: &mut String)  {
        let _ = res.read_to_string(s);

    }

    fn push_string_to_vec(&mut self, s: String) {
        let mut guard = self.json_strings.lock().unwrap();
        (*guard).push_front(s);
    }

    fn get_start(&mut self, res: &mut hyper::client::Response) -> String {
        let mut buff: &mut [u8] = &mut [0; 100];
        let _ = res.read_exact(&mut buff);
        String::from(str::from_utf8(buff).unwrap())

    }

    fn get_next_id(&mut self, s: &String) -> String{
        String::from(self.re.captures(s.as_str()).unwrap().at(0).unwrap())
    }

    fn send_next_id(&mut self,s: String) {
        let _ = self.msg_send.as_mut().unwrap().send(s);
    }

    fn get_next_response(&mut self) -> Option<hyper::client::Response>{
        let mut guard = self.responses.lock().unwrap();
        (*guard).pop_back()
    }
}

struct Crawler{
    responses: Arc<Mutex<VecDeque<hyper::client::Response>>>,
    client: Client,
    msg_send_to_downloader: mpsc::Sender<String>,
    recv_from_downloader: mpsc::Receiver<String>,
    next_id: String,
}

impl Crawler{
    fn init(&mut self){
        loop{
            let url = self.build_new_url();
            //println!("Crawler --> new url:{}",url);
            let now = Instant::now();
            let res = self.request(url.as_str());
            println!("{} Crawler --> request done in {},{}s",time::at(time::get_time()).ctime(), now.elapsed().as_secs(),now.elapsed().subsec_nanos());
            self.push_response(res);
            //println!("Crawler --> pushed response");
            self.notify_downloader();
            //println!("Crawler --> notified Downloader");

            self.next_id = self.recv_from_downloader.recv().unwrap();
            //println!("Crawler --> received new next id");

        }
    }

    fn request(&self, url: &str) -> hyper::client::Response {
        self.client.get(url).send().unwrap()
    }

    fn build_new_url(&self) -> String{
        let mut url = String::from("http://api.pathofexile.com/public-stash-tabs");
        url.push_str("?id=");
        url.push_str(self.next_id.as_str());
        url
    }

    fn notify_downloader(&mut self){
        self.msg_send_to_downloader.send("nD".to_string());
    }

    fn push_response(&mut self, res: hyper::client::Response) {
        let mut guard = self.responses.lock().unwrap();
        (*guard).push_front(res);
    }
}