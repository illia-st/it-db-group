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
    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) {
        log::debug!("performing simple echo server");
        let received_data = receiver.recv();
        // TODO: add ion data structures for command
        sender.send(received_data.as_slice());
    }
}