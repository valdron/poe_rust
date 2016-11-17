use postgres::{Connection, TlsMode};
use postgres::transaction::Transaction;
use parser::*;
use std::sync::mpsc::{Sender, Receiver};
use std::string::String;
use time;


pub struct PostgreSql {
        conn: Connection,
        from_parser: Receiver<RustStash>,
        logging: Sender<String>,
}

impl PostgreSql {
    pub fn new(from_p: Receiver<RustStash>, to_logger: Sender<String>) -> PostgreSql {

        PostgreSql{
            conn: Connection::connect("postgresql://postgres@localhost/poe_rust_dev", TlsMode::None).unwrap(),
            from_parser: from_p,
            logging: to_logger,
        }
    }


    pub fn init(&self) {
        loop {
            let stash: RustStash =  self.from_parser.recv().unwrap();
            let stash_id = stash.stash_id.clone();
            match self.insert_stash(stash) {
                Ok(x) => {self.logging.send(format!("{} | Writer\t\t\t--> Stash written successfully {}",
                                                   time::at(time::get_time()).ctime(),stash_id));}
                Err(e) => {
                    self.logging.send(format!("{} | Writer\t\t\t--> Error: {:?} on stash {} ",
                                              time::at(time::get_time()).ctime(),e,stash_id));
                }
            };

        }
    }
    /*
     *      tries to write complete stash in one transaction
     *
     *
     */
    fn insert_stash(&self, stash: RustStash) -> Result<&str, String>{
        //new transaction
        let trans: Transaction = self.conn.transaction().unwrap();
        //write stash data
        let present: bool = self.is_present_stash(&stash, &trans).unwrap();
        match present{
            true => {match trans.execute("UPDATE stashes SET (acc_name, last_char_name, stash_type, stash_name, item_nr, is_public) = ($2, $3, $4, $5, $6, $7) WHERE stash_id = $1",
                                  &[&stash.stash_id,
                                  &stash.acc_name,
                                  &stash.last_char_name,
                                  &stash.stash_type,
                                  &stash.stash_name,
                                  &stash.is_public,
                                  &stash.item_nr]){
                Ok(_) => {},
                Err(y) => {let _ = self.logging.send(format!("{:?}",y));}
            }
            }, // execute statement update
            false => {
                match trans.execute("INSERT INTO stashes VALUES ($1, $2, $3, $4, $5, $6, $7)",
                              &[&stash.stash_id,
                                  &stash.acc_name,
                                  &stash.last_char_name,
                                  &stash.stash_type,
                                  &stash.stash_name,
                                  &stash.is_public,
                                  &stash.item_nr]){
                    Ok(_) => {},
                    Err(y) => {let _ = self.logging.send(format!("{:?}",y));}
                }


            }, // execute statement insert
        };

        //write items in loop
        for item in stash.items{
            if !present {
                match self.item_diff(&item, &trans).unwrap() {
                    true => match self.insert_item(&item, &trans) {
                        Ok(_) => {},
                        Err(x) => {
                            trans.finish();
                            return Err(x);
                        }
                    },
                    false => match self.update_item(&item, &trans) {
                        Ok(_) => {},
                        Err(x) => {
                            trans.finish();
                            return Err(x);
                        }
                    },
                }

            } else {
                match self.insert_item(&item, &trans) {
                    Ok(_) => {},
                    Err(x) => {
                        trans.finish();
                        return Err(x);
                    }
                }
            }
        }
        trans.set_commit();
        let _ = trans.finish();
        Ok("Transaction completed successfully!")



    }

    fn insert_item(&self, item: &RustItem, trans: &Transaction) -> Result<&str,String>{
        let itype: String = format!("{:?}",item.item_type);
        let stmt: String = format!("INSERT INTO {} VALUES ($1)", itype);
        match trans.execute(stmt.as_str(),&[&item]){
            Ok(x) => Ok("item written"),
            Err(x) => Err(format!("{:?}",x))
        }
    }

    fn update_item(&self, item: &RustItem, trans: &Transaction) -> Result<&str,String>{
        unimplemented!();
    }

    fn item_diff(&self, item: &RustItem, trans: &Transaction) -> Result<bool ,&str>{

        let itype: String = format!("{:?}",item.item_type);
        let stmt: String = format!("SELECT id FROM {} WHERE id = $1", itype);
        match trans.query(stmt.as_str(), &[&item.item_id]) {
            Ok(x) => Ok(x.is_empty()),
            Err(_) => Err("Error in Postgres item diff")
        }
    }

    //checks if the provided stash is already in the db
    fn is_present_stash(&self, stash: &RustStash, trans: &Transaction) -> Result<bool,String> {
        match trans.query("SELECT stash_id FROM stashes WHERE stash_id = $1",&[&stash.stash_id]) {
            Ok(ref x) => Ok(!x.is_empty()),
            Err(e) => Err(format!("Error in Postgres is_present_stash {:?}",e))
        }
    }


}
