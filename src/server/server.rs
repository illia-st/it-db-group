use std::rc::Rc;
use std::sync::Arc;
use threadpool::ThreadPool;
use crate::db_manager::DatabaseManager;
use crate::transport::builder::ConnectorBuilder;
use crate::transport::poller::Poller;
use super::db_manager_handler::DbManagerHandler;

pub struct Server {
    pool: ThreadPool,
}

const DB_MANAGER_ENDPOINT: &str = "tcp://0.0.0.0:4044";
impl Server {
    pub fn new(pool: ThreadPool) -> Self {
        Self {
            pool,
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
                .with_handler(Rc::new(DbManagerHandler::new(manager)))
                .build_replyer()
                .bind()
                .into_inner();
            Poller::default()
                .add(db_manager_connector)
                .poll(-1);
        })
    }
}