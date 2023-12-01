use env_logger::Env;
use threadpool::ThreadPool;
use server::server::Server;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    log::info!("initializing a new server instance");
    let pool = ThreadPool::new(3);
    Server::new(pool.clone()).run();

    pool.join();
}
