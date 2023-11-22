use db_manager::db_manager::DatabaseManager;

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
    // receive message in bytes and return response in bytes
    pub fn dispatch(&self, message: Vec<u8>) -> Vec<u8> {
        todo!("finish implementing dispatch")
    }
}
