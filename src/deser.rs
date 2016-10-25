use serde_json::de;
use std::thread;
use std::sync::{Arc, Mutex};
use downloader::Provider;
use JsonSite;
use std::time::Duration;

pub struct JsonSiteDeser {
    //Shared Vector for deserialized data
    json_sites: Arc<Mutex<Vec<JsonSite>>>,

}

impl JsonSiteDeser{
    // Create new Deserializer
    pub fn new() -> JsonSiteDeser {
        JsonSiteDeser{
            json_sites: Arc::new(Mutex::new(Vec::new())),
        }
    }
    //Start self -> spawns thread
    pub fn start(&mut self, prov: Provider) {

        //create Thread data
        let mut thr_struct = PoeDeser{
            json_sites: self.json_sites.clone(),
            jp: prov,
        };
        //spawn thread and start deserializerloop
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
//Thread data
struct PoeDeser{
    json_sites: Arc<Mutex<Vec<JsonSite>>>,
    //provides JsonStrings
    jp:  Provider,
}

impl PoeDeser{
    //Thread main
    fn init(&mut self) {

        loop{
            match self.jp.get_json_string(){
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
        // deserialize Json String to struct
    fn deserialize(&self, s: String) -> JsonSite {
        let site: JsonSite = de::from_str(s.as_str()).unwrap();
        site
    }
        // write to shared vector
    fn write_to_vec(&mut self, site: JsonSite){
        let mut guard = self.json_sites.lock().unwrap();
        (*guard).push(site);
    }
}