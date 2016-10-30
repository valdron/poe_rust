use serde_json::de;
use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use downloader::Provider;
use serde_types::JsonSite;
use std::time::{Duration,Instant};
use time;
use std::collections::VecDeque;


pub struct JsonSiteDeser {
    //Shared Vector for deserialized data
    json_sites: Arc<Mutex<VecDeque<JsonSite>>>,

}

impl JsonSiteDeser{
    // Create new Deserializer
    pub fn new() -> JsonSiteDeser {
        JsonSiteDeser{
            json_sites: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    //Start self -> spawns thread
    pub fn start(&mut self, prov: Provider, recv: mpsc::Receiver<String>, send: mpsc::Sender<String>) {

        //create Thread data
        let mut thr_struct = PoeDeser{
            notify_parser: send,
            receive_from: recv,
            json_sites: self.json_sites.clone(),
            jp: prov,
        };
        //spawn thread and start deserializerloop
        thread::spawn(move||{
            thr_struct.init();
        });
        println!("{} | JsonSiteDeser\t--> deser thread started",time::at(time::get_time()).ctime());


    }

    pub fn get_next_jsonsite(&mut self) -> Option<JsonSite> {
        let mut guard = self.json_sites.lock().unwrap();
        (*guard).pop_back()
    }
    pub fn get_buff_len(&mut self) -> usize{
        let guard = self.json_sites.lock().unwrap();
        (*guard).len()
    }
}
//Thread data
struct PoeDeser{
    json_sites: Arc<Mutex<VecDeque<JsonSite>>>,
    receive_from: mpsc::Receiver<String>,
    notify_parser: mpsc::Sender<String>,
    //provides JsonStrings
    jp:  Provider,
}

impl PoeDeser{
    //Thread main
    fn init(&mut self) {

        loop{
            let m = self.receive_from.recv();
            match self.jp.get_json_string(){
                None => {
                },
                Some(x) => {
                    let now = Instant::now();
                    let site = self.deserialize(x);
                    self.write_to_vec(site);
                    println!("{} | PoeDeser\t\t\t--> deserialized and pushed in {}.{}", time::at(time::get_time()).ctime(),now.elapsed().as_secs(),now.elapsed().subsec_nanos());
                    let _ = self.notify_parser.send(String::from("pushed"));
                }
            }

        }
    }
        // deserialize Json String to struct
    fn deserialize(&self, s: String) -> JsonSite {
        let site: JsonSite = de::from_str(s.as_str()).unwrap();
        site
    }
        // write to shared vector
    fn write_to_vec(&mut self, site: JsonSite){
        let mut guard = self.json_sites.lock().unwrap();
        (*guard).push_front(site);
    }
}