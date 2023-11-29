use bson::{Document, doc, Bson};
use bson::Bson::Binary;
use db_api::{Decoder, Encoder};
use db_api::client_req::ClientRequest;
use db_api::envelope::Envelope;
use db_manager::db_manager::DatabaseManager;
use mongodb::sync::Collection;
use db_api::db::DatabaseDTO;
use transport::connectors::core::{Handler, Receiver, Sender};

pub struct DbManagerHandler {
    manager: DatabaseManager,
    database: mongodb::sync::Database,
}

impl DbManagerHandler {
    pub fn new(manager: DatabaseManager, db_client: mongodb::sync::Client) -> Self {
        let database = db_client.database("databases");
        match database.create_collection("list_of_databases", None) {
            Ok(_) => log::debug!("collection has been created"),
            Err(err) => log::error!("got an error after creating a collection {}", err),
        }
        Self { 
            manager,
            database,
        }
    }
}

impl Handler for DbManagerHandler {
    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) -> Option<Vec<u8>> {
        log::debug!("performing simple echo server");
        let received_data = receiver.recv();
        let envelope = Envelope::decode(received_data.as_slice());
        let client_req = ClientRequest::decode(envelope.get_data().to_vec());
        let collection: Collection<Document> = self.database.collection("list_of_databases");
        log::debug!("client request {:?}", client_req);
        // need to return bytes from this match
        let result = match client_req.command_type.as_str() {
            "delete" => {
                let database_path = client_req.database_path.unwrap();
                let database_name = client_req.database_name.unwrap();
                let res = self.manager.delete_db(database_path.as_str(), database_name.as_str());
                if let Ok(()) = res {
                    let new_doc = doc! {
                        "name": database_name,
                    };
                    log::info!("insert res: {:?}", collection.delete_one(new_doc, None).unwrap());
                    log::debug!("delete success");
                    Envelope::new("delete", &[]).encode()
                } else {
                    log::debug!("error");
                    Envelope::new("error", res.err().unwrap().as_bytes()).encode()
                }
            },
            "open" => {
                let database_name = client_req.database_name.unwrap();
                // need to get the db from mongodb instead of reading it from the directory
                let query = doc! {
                    "name": database_name
                };
                let res = collection.find_one(query, None).unwrap();

                if let Some(res) = res {
                    let data: String = res.get("db").unwrap().to_string();
                    let db = DatabaseDTO::decode(data.split(",").map(|s| s.parse().unwrap()).collect());
                    log::debug!("open success: {:?}", db);
                    Envelope::new("open", db.encode().as_slice()).encode()
                } else {
                    log::debug!("error");
                    Envelope::new("error", "couldn't open db".as_bytes()).encode()
                }
            },
            "create" => {
                let database_name = client_req.database_name.unwrap();
                let database_path = client_req.database_path.unwrap();
                let res = self.manager.create_db(database_name.as_str(), database_path.as_str());
                if let Some(db) = self.manager.get_db() {
                    // we need to save it now and then we will be able to update the document by knowing db name
                    let data = db.encode().iter().map(|&b| b.to_string()).collect::<Vec<String>>().join(",");
                    let new_doc = doc! {
                        "name": db.name.clone(),
                        "db": data
                    };
                    log::info!("insert res: {:?}", collection.insert_one(new_doc, None).unwrap());
                    log::debug!("create success");
                    Envelope::new("create", db.encode().as_slice()).encode()
                } else {
                    log::debug!("error");
                    Envelope::new("error", res.err().unwrap().as_bytes()).encode()
                }
            },
            "close" => {
                self.manager.close_db().unwrap();
                match client_req.db_to_save {
                    None => {
                        log::debug!("close success");
                        Envelope::new("close", &[]).encode()
                    }
                    Some(db) => {
                        let data = db.encode().iter().map(|&b| b.to_string()).collect::<Vec<String>>().join(",");
                        let new_doc = doc! {
                            "name": db.name.clone(),
                            "db": data
                        };
                        let filter = doc! {
                            "name": db.name.clone()
                        };
                        match collection.find_one_and_replace(filter, new_doc, None) {
                            Ok(_) => {
                                log::debug!("close success");
                                Envelope::new("close", &[]).encode()
                            }
                            Err(err) => {
                                log::debug!("error");
                                Envelope::new("error", format!("{}", err).as_bytes()).encode()
                            }
                        }
                    }
                }
            },
            _ => panic!("other commands aren't supported"),
        };
        sender.send(result.as_slice());
        None
    }
}