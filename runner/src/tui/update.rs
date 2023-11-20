use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use shellwords;

use crate::app::App;
use crate::app::ClosedDatabaseAppState;
use crate::app::DatabaseState;
use crate::app::OpenedDatabaseAppState;

use crate::parser::get_parser;

pub fn update(app: &mut App, key_event: KeyEvent) {
    if let KeyCode::Char(char) = key_event.code {
        app.add_char_to_buffer(char);
    }

    match key_event.code {
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        },
        KeyCode::Char('w') => {
            if let DatabaseState::Opened(OpenedDatabaseAppState::ActiveTable) = app.get_database_state() {
                app.selsect_priv_row()
            }
            if let DatabaseState::Opened(OpenedDatabaseAppState::ActiveJoinResult) = app.get_database_state() {
                app.selsect_priv_row()
            }
        },
        KeyCode::Char('s') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                if let DatabaseState::Opened(_) = app.get_database_state() {
                    app.close_database(true);
                    return;
                }
            }

            if let DatabaseState::Opened(OpenedDatabaseAppState::ActiveTable) = app.get_database_state() {
                app.selsect_next_row()
            }
            if let DatabaseState::Opened(OpenedDatabaseAppState::ActiveJoinResult) = app.get_database_state() {
                app.selsect_next_row()
            }
        },
        KeyCode::Char('a') => {
            if let DatabaseState::Opened(OpenedDatabaseAppState::ActiveTable) = app.get_database_state() {
                app.selsect_priv_column()
            }
            if let DatabaseState::Opened(OpenedDatabaseAppState::ActiveJoinResult) = app.get_database_state() {
                app.selsect_priv_column()
            }
        },
        KeyCode::Char('d') => {
            if let DatabaseState::Opened(OpenedDatabaseAppState::ActiveTable) = app.get_database_state() {
                app.selsect_next_column()
            }
            if let DatabaseState::Opened(OpenedDatabaseAppState::ActiveJoinResult) = app.get_database_state() {
                app.selsect_next_column()
            }
        },
        KeyCode::Char('/') => {
            if let DatabaseState::Closed(ClosedDatabaseAppState::None) = app.get_database_state() {
                app.activete_closed_database_hood();
                app.clear_buffer();
            }
            if let DatabaseState::Opened(OpenedDatabaseAppState::None) = app.get_database_state() {
                app.activete_opened_database_hood();
                app.clear_buffer();
            }
        },
        KeyCode::Char('[') => {
            if let DatabaseState::Opened(OpenedDatabaseAppState::ActiveMenu) = app.get_database_state() {
                app.selsect_priv_table()
            }
        }
        KeyCode::Char(']') => {
            if let DatabaseState::Opened(OpenedDatabaseAppState::ActiveMenu) = app.get_database_state() {
                app.selsect_next_table()
            }
        }
        KeyCode::Esc => {
            if let DatabaseState::Closed(ClosedDatabaseAppState::ActiveHood(_)) = app.get_database_state() {
                app.deactivete_closed_database_hood();
                app.clear_buffer();
            }
            if let DatabaseState::Opened(OpenedDatabaseAppState::ActiveHood(_)) = app.get_database_state() {
                app.deactivete_opened_database_hood();
                app.clear_buffer();
            }
        }
        KeyCode::Enter => {
            // TODO: move this part to the server one. It can parse an actual command inside the server
            if let DatabaseState::Closed(ClosedDatabaseAppState::ActiveHood(_)) = app.get_database_state() {
                let words = shellwords::split(&app.get_buffer());
                let matches = get_parser().try_get_matches_from_mut(words.unwrap());

                match matches {
                    Ok(_) => {},
                    Err(e) => {
                        app.opening_database_error(format!("{}", e));
                        app.clear_buffer();
                        return;
                    },
                }

                match matches.unwrap().subcommand() {
                    Some(("create", args)) => {
                        if args.get_flag("table") {
                            app.opening_database_error("Trying to create a table. But no database is active".to_owned());
                        }
                        if args.get_flag("database") {
                            app.create_database(
                                args.get_one::<String>("name").unwrap().to_owned(),
                                args.get_one::<String>("database_path").unwrap().to_owned()
                            );
                        }
                    },
                    Some(("delete", args)) => {
                        if args.get_flag("table") {
                            app.opening_database_error("Trying to delete a table. But no database is active".to_owned());
                        }
                        if args.get_flag("database") {
                            app.delete_database(
                                args.get_one::<String>("database_path").unwrap().to_owned(),
                                args.get_one::<String>("name").unwrap().to_owned(),
                            );
                        }
                    },
                    Some(("open", args)) => {
                        app.open_database(
                            args.get_one::<String>("database_path").unwrap().to_owned(),
                            args.get_one::<String>("database_name").unwrap().to_owned()
                        );
                    },
                    _ => {
                        app.opening_database_error("Unsupported comand for this hood".to_owned());
                    },
                }

                app.clear_buffer();
            }

            if let DatabaseState::Opened(OpenedDatabaseAppState::ActiveHood(_)) = app.get_database_state() {
                let words = shellwords::split(&app.get_buffer());

                match words {
                    Ok(_) => {},
                    Err(e) => {
                        app.opened_database_error(format!("{}", e));
                        app.clear_buffer();
                        return;
                    },
                }

                let matches = get_parser().try_get_matches_from_mut(words.unwrap());

                match matches {
                    Ok(_) => {},
                    Err(e) => {
                        app.opened_database_error(format!("{}", e));
                        app.clear_buffer();
                        return;
                    },
                }

                match matches.unwrap().subcommand() {
                    Some(("create", args)) => {
                        if args.get_flag("table") {
                            app.create_table(
                                args.get_one::<String>("name").unwrap().to_owned(),
                                args.get_one::<String>("table_column_names").unwrap().to_owned(),
                                args.get_one::<String>("table_types").unwrap().to_owned()
                            );
                        }
                        if args.get_flag("database") {
                            app.opened_database_error("Trying to create a database. But database is active".to_owned());
                        }
                    },
                    Some(("add", args)) => {
                        app.add_row(
                            args.get_one::<String>("table_name").unwrap().to_owned(),
                            args.get_one::<String>("row_value").unwrap().to_owned(),
                        );
                    }
                    Some(("delete", args)) => {
                        if args.get_flag("table") {
                            app.delete_table(
                                args.get_one::<String>("name").unwrap().to_owned(),
                            );
                        }
                        if args.get_flag("database") {
                            app.opened_database_error("Trying to delete a database. But this database is active".to_owned());
                        }
                    },
                    Some(("close", args)) => {
                        app.close_database(args.get_flag("save"))
                    },
                    Some(("remove", args)) => {
                        app.delete_row(
                            args.get_one::<String>("table_name").unwrap().to_owned(),
                            args.get_one::<String>("row_index").unwrap().to_owned()
                        )
                    },
                    Some(("rename", args)) => {
                        app.rename_row(
                            args.get_one::<String>("table_name").unwrap().to_owned(),
                            args.get_one::<String>("table_column_names").unwrap().to_owned()
                        )
                    },
                    Some(("join", args)) => {
                        app.get_join_result(
                            args.get_one::<String>("left_table_name").unwrap().to_owned(),
                            args.get_one::<String>("right_table_name").unwrap().to_owned(),
                            args.get_one::<String>("column_name").unwrap().to_owned()
                        )
                    },
                    _ => {
                        app.opened_database_error("Unsupported comand for this hood".to_owned());
                    },
                }

                app.clear_buffer();
            }

            if let DatabaseState::Opened(OpenedDatabaseAppState::ActiveMenu) = app.get_database_state() {
                app.show_table();
                app.activete_opened_database_active_table();
            }
        },
        KeyCode::Backspace => {
            app.remove_last_char_from_the_buffer();
        },
        KeyCode::Left => {
            if let DatabaseState::Opened(_) = app.get_database_state() {
                app.activete_opened_database_active_menu();
                app.clear_buffer();
            }
        },
        KeyCode::Right => {
            if let DatabaseState::Opened(_) = app.get_database_state() {
                app.activete_opened_database_hood();
                app.clear_buffer();
            }
        },
        KeyCode::Up => {
            if let DatabaseState::Opened(_) = app.get_database_state() {
                app.activete_opened_database_hood();
                app.clear_buffer();
            }
        },
        KeyCode::Down => {
            if let DatabaseState::Opened(_) = app.get_database_state() {
                app.activete_opened_database_active_table();
                app.clear_buffer();
            }
        },
        _ => {},
    };
}