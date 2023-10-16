use std::ops::Deref;

use rand::Rng;
use ratatui::style::Stylize;
use ratatui::text::Span;
use ratatui::text::Line;

use ratatui::prelude::Rect;
use ratatui::prelude::Constraint;
use ratatui::prelude::Direction;
use ratatui::prelude::Layout;

use ratatui::widgets::Cell;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Borders;
use ratatui::widgets::BorderType;
use ratatui::widgets::Block;

use ratatui::layout::Alignment;

use ratatui::style::Color;
use ratatui::style::Style;

use rand;
use ratatui::widgets::Row;
use ratatui::widgets::Table;
use ratatui::widgets::Wrap;

use crate::app::App;
use crate::app::ClosedDatabaseAppState;
use crate::app::DatabaseState;

use crate::tui::Frame;

pub fn render(app: &mut App, f: &mut Frame) {
    match app.get_database_state() {
        DatabaseState::Closed(_) => {
            render_default_screen(f, app);
        },
        DatabaseState::Opened(_) => {
            render_main_screen(f, app);
        },
    }
}

fn render_default_screen_body(f: &mut Frame, layout : Rect, color: Color) {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    f.render_widget(
        Paragraph::new("\n\nWelcome to the DB paradise!\nPress `Ctrl-C` to stop running.".to_string())
            .block(
                Block::default()
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(color)),
            )
            .style(Style::default().fg(Color::Rgb(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255))).bold())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center),
            layout,
    )
}

fn render_screen_hood(f: &mut Frame, layout: Rect, color: Color, text: String) {
    f.render_widget(
        Paragraph::new(text)
            .block(
                Block::default()
                    .title(
                        Line::from(vec![
                            Span::styled(" Enter Input Mode ", Style::default().fg(color)),
                            Span::styled("(Press ", Style::default().fg(Color::DarkGray)),
                            Span::styled("/", Style::default().fg(Color::White)),
                            Span::styled(" to enter Hood, ", Style::default().fg(Color::DarkGray)),
                            Span::styled("Esc", Style::default().fg(Color::White)),
                            Span::styled(" to leave) ", Style::default().fg(Color::DarkGray)),
                        ])
                    )
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick),
            )
            .style(Style::default().fg(color))
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center),
            layout,
    )
}

fn render_active_menu(f: &mut Frame, layout: Rect, color: Color, db_name: String, table_names: Vec<String>, index: usize) {
    let mut lines: Vec<Line> = Vec::new();
    for (i, table_name) in table_names.iter().enumerate() {
        if i == index {
            lines.push(Line::from(Span::styled(table_name, Style::default().fg(Color::Cyan).bold())))
        } else {
            lines.push(Line::from(Span::styled(table_name, Style::default().fg(Color::DarkGray))))
        }
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled("Whoops, no tables in this database :(", Style::default().fg(Color::Red).bold().italic())))
    }
    
    f.render_widget(
        Paragraph::new(lines)
            .block(
                Block::default()
                    .title(format!(" {} ", db_name))
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(color)),
            )
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left),
            layout,
    )
}

fn render_active_table(f: &mut Frame, layout: Rect, color: Color, table_result: Result<core::table::Table, String>, selected_row: usize, selected_column: usize) {
    if let Err(ref e) = table_result {
        f.render_widget(
            Paragraph::new(e.deref())
                .block(
                    Block::default()
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Thick)
                        .border_style(Style::default().fg(color)),
                )
                .wrap(Wrap { trim: true })
                .alignment(Alignment::Left)
                .style(Style::default().fg(Color::Red).bold().italic()),
                layout,
        );
        return;
    }

    let table = table_result.unwrap();

    let mut header_content: Vec<String> = Vec::new();
    let mut widths = Vec::new();
    for column_header in table.get_columns() {
        widths.push(Constraint::Length(table.get_max_column_len(&column_header) as u16));
        header_content.push(column_header);
    }
    let table_header = Row::new(header_content);

    let mut table_content = Vec::new();
    
    let mut row_number = 0;
    for row in table.get_rows().iter() {
        let mut row_content = Vec::new();
        
        let mut cell_number = 0;
        for cell in row.get_values() {
            let cell_content =
                match cell.get_value() {
                    core::types::ValueType::Int(int) => {
                        int.get_value().to_string()
                    },
                    core::types::ValueType::Str(str) => {
                        str.get_value().to_owned()
                    },
                    core::types::ValueType::Real(real) => {
                        real.get_value().to_string()
                    },
                    core::types::ValueType::Pic(_picture) => {
                        "picture".to_owned()
                    },
                    core::types::ValueType::Char(char) => {
                        char.get_value().to_string()
                    },
                    core::types::ValueType::Date(date) => {
                        date.get_value().to_string()
                    },
                    core::types::ValueType::Email(email) => {
                        email.get_value().as_str().to_owned()
                    },
                };
            
            let mut cell = Cell::from(cell_content);
            if selected_row == row_number && selected_column == cell_number {
                cell = cell.style(Style::default().fg(color));
            }

            row_content.push(cell);
            
            cell_number += 1;
        }
        table_content.push(Row::new(row_content));
        
        row_number += 1;
    }

    let table = 
        Table::new(table_content)
            .header(table_header)
            .block(
                Block::default()
                    .title(format!(" {} ", table.get_name()))
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(color)),
            )
            .widths(&widths)
            .column_spacing(3);

    f.render_widget(table, layout);
}

fn render_default_screen(f: &mut Frame, app: &mut App) {
    if let DatabaseState::Closed(state) = app.get_database_state() {
        let layout = 
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(80),
                Constraint::Percentage(20)
            ].as_ref())
            .split(f.size());
        let err_layout = 
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(80)
            ].as_ref())
            .split(f.size());
    
        match state {
            ClosedDatabaseAppState::ActiveHood(e) => {
                if e.is_empty() {
                    render_default_screen_body(f, layout[0], Color::White);
                    render_screen_hood(f, layout[1], Color::Cyan, app.get_buffer());
                } else {
                    render_default_screen_body(f, err_layout[0], Color::White);
                    render_screen_hood(f, err_layout[1], Color::Red, e);
                }
            },
            ClosedDatabaseAppState::None => {
                render_default_screen_body(f, layout[0], Color::White);
                render_screen_hood(f, layout[1], Color::White, "".to_owned());
            },
        }  
    }
}

fn render_main_screen(f: &mut Frame, app: &mut App) {
    if let DatabaseState::Opened(state) = app.get_database_state() {
        let layout = 
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(80)
            ].as_ref())
            .split(f.size());
        
        let inner_layout = 
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(80)
            ].as_ref())
            .split(layout[1]);
        let err_inner_layout = 
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(80),
                Constraint::Percentage(20)
            ].as_ref())
            .split(layout[1]);

        match state {
            crate::app::OpenedDatabaseAppState::ActiveHood(e) => {
                render_active_menu(f, layout[0], Color::White, app.get_database_name(), app.get_table_list(), app.get_selected_table_index());
                if e.is_empty() {
                    render_screen_hood(f, inner_layout[0], Color::Cyan, app.get_buffer());
                    render_active_table(f, inner_layout[1], Color::White, app.get_current_table(), app.get_selected_row_index(), app.get_selected_column_index());
                } else {
                    render_screen_hood(f, err_inner_layout[0], Color::Red, e);
                    render_active_table(f, err_inner_layout[1], Color::White, app.get_current_table(), app.get_selected_row_index(), app.get_selected_column_index());
                }
            },
            crate::app::OpenedDatabaseAppState::ActiveMenu => {
                render_active_menu(f, layout[0], Color::Cyan, app.get_database_name(), app.get_table_list(), app.get_selected_table_index());
                render_screen_hood(f, inner_layout[0], Color::White, "".to_owned());
                render_active_table(f, inner_layout[1], Color::White, app.get_current_table(), app.get_selected_row_index(), app.get_selected_column_index());
            },
            crate::app::OpenedDatabaseAppState::ActiveTable => {
                render_active_menu(f, layout[0], Color::White, app.get_database_name(), app.get_table_list(), app.get_selected_table_index());
                render_screen_hood(f, inner_layout[0], Color::White, "".to_owned());
                render_active_table(f, inner_layout[1], Color::Cyan, app.get_current_table(), app.get_selected_row_index(), app.get_selected_column_index());
            },
            crate::app::OpenedDatabaseAppState::None => {
                render_active_menu(f, layout[0], Color::White, app.get_database_name(), app.get_table_list(), app.get_selected_table_index());
                render_screen_hood(f, inner_layout[0], Color::White, "".to_owned());
                render_active_table(f, inner_layout[1], Color::White, app.get_current_table(), app.get_selected_row_index(), app.get_selected_column_index());
            },
        } 
    }
}