use crate::db_api::client_req::ClientRequest;
use crate::db_api::{Decoder, Encoder};
use crate::db_api::envelope::Envelope;
use crate::db_manager::DatabaseManager;

#[derive(Debug)]
pub struct CommandDispatcher {
    pub database_manager: DatabaseManager,
}


impl Default for CommandDispatcher {
    fn default() -> Self {
        Self {
            database_manager: DatabaseManager::new(),
        }
    }
}
impl CommandDispatcher {
    pub fn dispatch(&self, message: Vec<u8>) -> Vec<u8> {
        let envelope = Envelope::decode(message.as_slice());
        let client_req = ClientRequest::decode(envelope.get_data().to_vec());
        log::debug!("client request {:?}", client_req);
        match client_req.command_type.as_str() {
            "create" => {
                let database_path = client_req.database_path.unwrap();
                let database_name = client_req.database_name.unwrap();
                let res = self.database_manager.create_db(database_name.as_str(), database_path.as_str());
                if let Some(db) = self.database_manager.get_db() {
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
                let res = self.database_manager.delete_db(database_path.as_str(), database_name.as_str());
                if res.is_ok() {
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
                let res = self.database_manager.read_db_from_directory(database_path.as_str(), database_name.as_str());
                if let Some(db) = self.database_manager.get_db() {
                    log::debug!("open success");
                    Envelope::new("open", db.encode().as_slice()).encode()
                } else {
                    log::debug!("error");
                    Envelope::new("error", res.err().unwrap().as_bytes()).encode()
                }
            },
            "close" => {
                let save = if let Some(db_to_save) = client_req.db_to_save {
                    self.database_manager.set_db_dto(db_to_save);
                    true
                } else {
                    false
                };
                let res = self.database_manager.close_db(save);
                if res.is_ok() {
                    log::debug!("close success");
                    Envelope::new("close", &[]).encode()
                } else {
                    log::debug!("error");
                    Envelope::new("error", res.err().unwrap().as_bytes()).encode()
                }
            },
            _ => panic!("other commands aren't supported"),
        }
    }
}
