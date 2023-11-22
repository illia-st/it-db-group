use db_api::{Decoder, Encoder};
use db_api::client_req::ClientRequest;
use db_api::envelope::Envelope;
use db_manager::db_manager::DatabaseManager;
use transport::connectors::core::{Handler, Receiver, Sender};

pub struct DbManagerHandler {
    #[allow(dead_code)]
    manager: DatabaseManager,
}

impl DbManagerHandler {
    pub fn new(manager: DatabaseManager) -> Self {
        Self { manager }
    }
}

impl Handler for DbManagerHandler {
    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) -> Option<Vec<u8>> {
        log::debug!("performing simple echo server");
        let received_data = receiver.recv();
        let envelope = Envelope::decode(received_data.as_slice());
        let client_req = ClientRequest::decode(envelope.get_data().to_vec());
        log::debug!("client request {:?}", client_req);
        // need to return bytes from this match
        let result = match client_req.command_type.as_str() {
            "create" => {
                let database_path = client_req.database_path.unwrap();
                let database_name = client_req.database_name.unwrap();
                let res = self.manager.create_db(database_name.as_str(), database_path.as_str());
                if let Some(db) = self.manager.get_db() {
                    log::debug!("create success");
                    Envelope::new("create", db.encode().as_slice()).encode()
                } else {
                    log::debug!("error");
                    Envelope::new("error", res.err().unwrap().as_bytes()).encode()
                }
            },
            "delete" => {
                let database_path = client_req.database_path.unwrap();
                let database_name = client_req.database_name.unwrap();
                let res = self.manager.delete_db(database_name.as_str(), database_path.as_str());
                if let Ok(()) = res {
                    log::debug!("delete success");
                    Envelope::new("delete", &[]).encode()
                } else {
                    log::debug!("error");
                    Envelope::new("error", res.err().unwrap().as_bytes()).encode()
                }
            },
            "open" => {
                let database_path = client_req.database_path.unwrap();
                let database_name = client_req.database_name.unwrap();
                let res = self.manager.create_db(database_name.as_str(), database_path.as_str());
                if let Some(db) = self.manager.get_db() {
                    log::debug!("open success");
                    Envelope::new("open", db.encode().as_slice()).encode()
                } else {
                    log::debug!("error");
                    Envelope::new("error", res.err().unwrap().as_bytes()).encode()
                }
            },
            "close" => {
                let save = client_req.save.unwrap();
                let res = self.manager.close_db(save);
                if let Ok(()) = res {
                    log::debug!("close success");
                    Envelope::new("close", &[]).encode()
                } else {
                    log::debug!("error");
                    Envelope::new("error", res.err().unwrap().as_bytes()).encode()
                }
            },
            _ => panic!("other commands aren't supported"),
        };
        sender.send(result.as_slice());
        None
    }
}