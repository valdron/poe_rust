use serde_json::de;
use std::thread;
use std::sync::{Arc, Mutex};
use downloader::Downloader;
use JsonSite;
use std::time::Duration;

pub struct JsonSiteDeser {
    json_sites: Arc<Mutex<Vec<JsonSite>>>,

}

impl JsonSiteDeser{
    pub fn new() -> JsonSiteDeser {
        JsonSiteDeser{
            json_sites: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn start(&mut self) {
        let mut thr_struct = PoeDeser{
            json_sites: self.json_sites.clone(),
            dl: Downloader::new(),
        };

        thread::spawn(move||{
            thr_struct.init();
        });
        println!("JsonSiteDeser --> deser thread started");


    }

    pub fn get_next_jsonsite(&mut self) -> Option<JsonSite> {
        let mut guard = self.json_sites.lock().unwrap();
        (*guard).pop()
    }
    pub fn get_buff_len(&mut self) -> usize{
        let guard = self.json_sites.lock().unwrap();
        (*guard).len()
    }
}

struct PoeDeser{
    json_sites: Arc<Mutex<Vec<JsonSite>>>,
    dl: Downloader,
}

impl PoeDeser{
    fn init(&mut self) {
        print!("PoeDeser --> Starting Downloader");

        self.dl.start();
        println!("PoeDeser --> done");

        loop{
            match self.dl.get_json_string(){
                None => {
                    println!("PoeDeser --> parking for 5000ms");
                    thread::park_timeout(Duration::from_millis(5000));
                },
                Some(x) => {
                    let site = self.deserialize(x);
                    self.write_to_vec(site);
                    println!("PoeDeser --> deserialized and pushed")
                }
            }

        }
    }
    fn deserialize(&self, s: String) -> JsonSite {
        let site: JsonSite = de::from_str(s.as_str()).unwrap();
        site
    }

    fn write_to_vec(&mut self, site: JsonSite){
        let mut guard = self.json_sites.lock().unwrap();
        (*guard).push(site);
    }
}