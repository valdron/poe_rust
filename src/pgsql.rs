use postgres::{Connection, TlsMode};
use parser::*;
use std::sync::mpsc::{Sender, Receiver};


struct PostgreSql {
        conn: Connection,
        from_parser: Receiver<RustStash>,
        logging: Sender<String>,
}

impl PostgreSql {
    pub fn new(from_p: Receiver<RustStash>, to_logger: Sender<String>) -> PostgreSql {
        PostgreSql{
            conn: Connection::connect("postgres://postgres@localhost", TlsMode::None).unwrap(),
            from_parser: from_p,
            logging: to_logger,
        }
    }
}
