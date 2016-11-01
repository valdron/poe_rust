use std::sync::mpsc;


pub struct Logger{
    receive: mpsc::Receiver<String>,
}

impl Logger {
    pub fn new(recv: mpsc::Receiver<String>) -> Logger{
            Logger{
                receive: recv,
            }
    }

    pub fn init(&self) {
        loop {
            let msg = self.receive.recv();
            match msg {
                Err(ref x) => println!("Logger recv-Err({}) ", x),
                Ok(ref x) if x == "stop" => break,
                Ok(ref x) => println!("{}", x),
            }

        }
    }

}