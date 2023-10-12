use ratatui::prelude::CrosstermBackend;
use ratatui::prelude::Terminal;

use anyhow::Result;
use runner::app::App;
use runner::tui::event::{Event, EventHandler};
use runner::tui::Tui;
use runner::tui::update::update;


fn main() -> Result<()> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    while !app.should_quit() {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick => {},
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {},
            Event::Resize(_, _) => {},
        };
    }

    tui.exit()?;
    Ok(())
}