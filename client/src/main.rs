use std::rc::Rc;
use std::sync::Arc;
use ratatui::prelude::CrosstermBackend;
use ratatui::prelude::Terminal;

use anyhow::Result;
use client::app::App;
use client::tui::event::{Event, EventHandler};
use client::tui::Tui;
use client::tui::update::update;
use transport::connectors::builder::ConnectorBuilder;
use transport::connectors::core::{Handler, Receiver, Sender, Socket};
use transport::connectors::poller::Poller;

const SERVER_ENDPOINT: &str = "tcp://0.0.0.0:4044";

struct Mediator { }
impl Handler for Mediator {
    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) -> Option<Vec<u8>>{
        log::debug!("dummy has received some data");
        Some(receiver.recv())
    }
}

fn main() -> Result<()> {
    let mut app = App::new();
    let context = Arc::new(zmq::Context::new());
    let connector = ConnectorBuilder::new()
        .with_context(context)
        .with_endpoint(SERVER_ENDPOINT.to_string())
        .with_handler(Rc::new(Mediator { }))
        .build_requester()
        .connect()
        .into_inner();
    let mut poller = Poller::new();
    poller.add(connector.clone() as Rc<dyn Socket>);

    app.set_connector(connector.clone());

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;


    // TODO: run a client here, give the connector as a parameter to the improvised handler

    while !app.should_quit() {
        // we need to work in a single thread with zmq
        // After each request we will run a poll for a single item_poll_count
        // it means we will be blocked until server returns something to us
        // looks legit
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick => {},
            Event::Key(key_event) => {
                update(&mut app, key_event);
                // poll will return the result if it is there
                if app.is_sent_req() {
                    let server_reply = poller.poll(1); // -> Option<Vec<u8>> (so, it doesn't necessary need to return anything )
                    app.update_state_by_server_reply(server_reply);
                }
                // then result is being returned and we can pass it to the app to save the result
            },
            Event::Mouse(_) => {},
            Event::Resize(_, _) => {},
        };
    }

    tui.exit()?;
    Ok(())
}