use env_logger::Env;
use server::ws_server::WsServer;
const WS_SERVER: &str = "0.0.0.0:9091";
fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    log::info!("initializing a new server instance");
    // creating
    WsServer::default()
        // binding
        .bind(WS_SERVER.to_string())
        // and polling the ws server. -1 means that the server is going to work forever
        .poll(-1);
}
