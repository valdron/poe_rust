use serde_json;
use std::sync::mpsc;
use serde_types::JsonSite;
use std::time::Instant;
use time;

//Thread data
pub struct PoeDeser{
    receive_from: mpsc::Receiver<String>,
    notify_parser: mpsc::Sender<JsonSite>,
    logging: mpsc::Sender<String>,
    //provides JsonStrings
}

impl PoeDeser{
    pub fn new(to_parser: mpsc::Sender<JsonSite>, from_downloader: mpsc::Receiver<String>, to_logger: mpsc::Sender<String>) -> PoeDeser {
        PoeDeser{
            logging: to_logger,
            receive_from: from_downloader,
            notify_parser: to_parser,
        }
    }
    //Thread main
    pub fn init(&mut self) {

        loop{
            let m = self.receive_from.recv();
            match m{
                Err(e) => { let _= self.logging.send(format!("{} | PoeDeser\t\t\t--> Err : {:?}", time::at(time::get_time()).ctime(),e));
                },
                Ok(x) => {
                    let now = Instant::now();
                    let site = self.deserialize(x);
                    match site {
                        Ok(x) => {
                            let _= self.notify_parser.send(x);
                            let _= self.logging.send(format!("{} | PoeDeser\t\t\t--> deserialized and sent in {}.{}",
                                                      time::at(time::get_time()).ctime(), now.elapsed().as_secs(),
                                                      now.elapsed().subsec_nanos()));
                        }
                        Err(e) => {
                            let _= self.logging.send(format!("{} | PoeDeser\t\t\t--> Err : {:?}", time::at(time::get_time()).ctime(),e));
                        }
                }
                }
            }

        }
    }
        // deserialize Json String to struct
    fn deserialize(&self, s: String) -> Result<JsonSite, serde_json::Error> {

        let site = serde_json::de::from_str(s.as_str());
        site
    }

}