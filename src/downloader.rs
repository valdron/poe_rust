extern crate hyper;
extern crate regex;

use regex::Regex;
use std::thread::*;
use hyper::client::{Response, Client};
use std::sync::mpsc;
use std::str;
use std::io::prelude::*;
use std::time::{Instant, Duration};
use time;


pub struct Downloader{
    msg_send: Option<mpsc::Sender<String>>,
    msg_receive: Option<mpsc::Receiver<Response>>,
    send_to_deser: mpsc::Sender<String>,
    logging: mpsc::Sender<String>,
    re: Regex,
}

impl Downloader{
    pub fn new(sender: mpsc::Sender<String>, to_logger: mpsc::Sender<String>) -> Downloader {
        Downloader {
            logging: to_logger,
            msg_send: None,
            msg_receive: None,
            send_to_deser: sender,
            re: Regex::new("[0-9]*-[0-9]*-[0-9]*-[0-9]*-[0-9]*").unwrap(),
        }
    }
    fn start_crawler(&mut self, mut c: Crawler){

        let _ = spawn(move ||{
            c.init();
        });
    }
    pub fn init(&mut self, start_id: String){
        let (send, crecv) = mpsc::channel();
        let (csend, recv) = mpsc::channel();
        self.msg_send = Some(send);
        self.msg_receive = Some(recv);

        let c = Crawler{
            logging: self.logging.clone(),
            client: Client::new(),
            msg_send_to_downloader: csend,
            recv_from_downloader: crecv,
            next_id: start_id,
        };
        self.start_crawler(c);
        loop{
            let mut res = self.msg_receive.as_mut().unwrap().recv().unwrap();
            //println!("Downloader --> got msg from Crawler");
            //let mut res: hyper::client::Response = self.get_next_response().unwrap();
            //println!("Downloader --> got next response");
            let mut start_string: String = self.get_start(&mut res);
            let next_id = self.get_next_id(&start_string);
            //println!("Downloader --> got next id:{}",next_id);
            self.send_next_id(next_id);
            //println!("Downloader --> sent next id to crawler");
            let now = Instant::now();
            self.read_rest_to_str(res, &mut start_string);
            let _ = self.send_to_deser.send(start_string);
            let _ = self.logging.send(format!("{} | Downloader\t\t--> read to string and pushed in {},{}s",
                                      time::at(time::get_time()).ctime(),
                                      now.elapsed().as_secs(),
                                      now.elapsed().subsec_nanos()));

        }

    }

    fn read_rest_to_str(&self, mut res: Response,  s: &mut String)  {
        let _ = res.read_to_string(s);

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
}

struct Crawler{
    client: Client,
    msg_send_to_downloader: mpsc::Sender<Response>,
    recv_from_downloader: mpsc::Receiver<String>,
    next_id: String,
    logging: mpsc::Sender<String>,
}

impl Crawler{
    fn init(&mut self){
        let mintime = Duration::from_secs(1);
        loop{
            let url = self.build_new_url();
            let now = Instant::now();
            let res = self.request(url.as_str());

            let _= self.logging.send(format!("{} | Crawler\t\t\t--> request done in {}.{}s",
                                      time::at(time::get_time()).ctime(),
                                      now.elapsed().as_secs(),
                                      now.elapsed().subsec_nanos()));
            let _= self.msg_send_to_downloader.send(res);

            let elapsed: Duration = now.elapsed();
            if mintime > elapsed {
                let dur: Duration = mintime-elapsed;
                let _= self.logging.send(format!("{} | Crawler\t\t\t--> am to fast parking for {}.{}",
                                          time::at(time::get_time()).ctime(),
                                          dur.as_secs(),
                                          dur.subsec_nanos()));
                park_timeout(dur);
            }
            self.next_id = self.recv_from_downloader.recv().unwrap();
        }
    }

    fn request(&mut self, url: &str) -> hyper::client::Response {
        match self.client.get(url).send() {
            Ok(x) => {
                return x;
            }

            _ => {
                loop {
                    let _= self.logging.send(format!("{} | Crawler\t\t\t--> connection closed, trying to reopen",
                                              time::at(time::get_time()).ctime()));
                    self.client = Client::new();
                    match self.client.get(url).send() {
                        Ok(x) => {
                            let _= self.logging.send(format!("{} | Crawler\t\t\t--> reopening successful",
                                                      time::at(time::get_time()).ctime()));
                            return x;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn build_new_url(&self) -> String{
        let mut url = String::from("http://api.pathofexile.com/public-stash-tabs");
        url.push_str("?id=");
        url.push_str(self.next_id.as_str());
        url
    }
}