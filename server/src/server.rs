use std::rc::Rc;
use std::sync::Arc;
use threadpool::ThreadPool;
use db_manager::db_manager::DatabaseManager;
use transport::connectors::builder::ConnectorBuilder;
use transport::connectors::poller::Poller;
use crate::db_manager_handler::DbManagerHandler;
use mongodb::sync::Client;

pub struct Server {
    pool: ThreadPool,
    mongo: Client,
}

const DB_MANAGER_ENDPOINT: &str = "tcp://0.0.0.0:4044";
const MONGO_DB_ENDPOINT: &str = "mongodb://admin:adminpassword@localhost:27017";
impl Server {
    pub fn new(pool: ThreadPool) -> Self {
        let mongo = Client::with_uri_str(MONGO_DB_ENDPOINT).unwrap();
        Self {
            pool,
            mongo,
        }
    }

    pub fn run(self) {
        self.pool.execute(move || {
            log::debug!("starting a new server on {DB_MANAGER_ENDPOINT}");
            let manager = DatabaseManager::default();
            let context = Arc::new(zmq::Context::default());
            let db_manager_connector = ConnectorBuilder::new()
                .with_context(context.clone())
                .with_endpoint(DB_MANAGER_ENDPOINT.to_string())
                .with_handler(Rc::new(DbManagerHandler::new(manager, self.mongo)))
                .build_replyer()
                .bind()
                .into_inner();
            Poller::default()
                .add(db_manager_connector)
                .poll(-1);
        })
    }
}