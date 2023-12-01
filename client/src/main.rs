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

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;


    // TODO: run a client here, give the connector as a parameter to the improvised handler

    while !app.should_quit() {
        tui.draw(&mut app)?;                                         

        match tui.events.next()? {
            Event::Tick => {},
            Event::Key(key_event) => {
                update(&mut app, key_event);
            },
            Event::Mouse(_) => {},
            Event::Resize(_, _) => {},
        };
    }

    tui.exit()?;
    Ok(())
}