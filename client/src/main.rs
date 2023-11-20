use std::rc::Rc;
use std::sync::Arc;
use ratatui::prelude::CrosstermBackend;
use ratatui::prelude::Terminal;

use anyhow::Result;
use client::app::App;
use client::tui::event::{Event, EventHandler};
use client::tui::Tui;
use client::tui::update::{CommandHandler, update};
use transport::connectors::builder::ConnectorBuilder;
use transport::connectors::core::{Sender, Socket};
use transport::connectors::poller::Poller;

const SERVER_ENDPOINT: &str = "tcp://0.0.0.0:4044";

fn main() -> Result<()> {
    let context = Arc::new(zmq::Context::new());
    let connector = ConnectorBuilder::new()
        .with_context(context)
        .with_endpoint(SERVER_ENDPOINT.to_string())
        .with_handler(Rc::new(CommandHandler { }))
        .build_requester()
        .connect()
        .into_inner();
    let mut poller = Poller::new();
    poller.add(connector.clone() as Rc<dyn Socket>);

    let mut app = App::new(connector.clone() as Rc<dyn Sender>);

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
                poller.poll(1);
            },
            Event::Mouse(_) => {},
            Event::Resize(_, _) => {},
        };
    }

    tui.exit()?;
    Ok(())
}