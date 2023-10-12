use std::ops::Deref;

use db_manager::db_manager::DatabaseManager;
use core;

pub enum Action {
    Tick,
    Quit,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseState {
    Closed(ClosedDatabaseAppState),
    Opened(OpenedDatabaseAppState),
}

impl Default for DatabaseState {
    fn default() -> Self {
        DatabaseState::Closed(ClosedDatabaseAppState::None)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum ClosedDatabaseAppState {
    ActiveHood(String),
    #[default]
    None
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum OpenedDatabaseAppState {
    ActiveHood(String),
    ActiveMenu,
    ActiveTable,
    #[default]
    None
}

#[derive(Debug, Default)]
pub struct App {
    should_quit: bool,
    database_state: DatabaseState,
    buffer: String,
    database_manager: DatabaseManager,
    displayed_table: usize,
    selected_table: usize
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn get_database_state(&self) -> DatabaseState {
        self.database_state.clone()
    }
    pub fn create_database(&mut self, name: String, database_path: String) {
        let result = self.database_manager.create_db(&name, &database_path);
        match result {
            Ok(_) => {
                self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::None)
            },
            Err(e) => {
                self.opening_database_error(e);
            },
        }
    }
    pub fn open_database(&mut self, database_path: String) {
        let result = self.database_manager.read_db_from_directory(&database_path);
        match result {
            Ok(_) => {
                self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::None)
            },
            Err(e) => {
                self.opening_database_error(e);
            },
        }
    }
    pub fn close_database(&mut self, need_to_save: bool) {
        let result = self.database_manager.close_db(need_to_save);
        match result {
            Ok(_) => {
                self.database_state = DatabaseState::Closed(ClosedDatabaseAppState::None)
            },
            Err(e) => {
                self.opening_database_error(e);
            },
        }
    }

    pub fn create_table(&mut self, table_name: String, columns: String, data_types: String) {
        let column_names = columns.split_terminator(';').collect::<Vec<&str>>();
        let column_data = data_types.split_terminator(';').collect::<Vec<&str>>();

        let result = self.database_manager.create_table(
            table_name.deref(),
            column_names, 
            column_data
        );
        match result {
            Ok(_) => {
                self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::None)
            },
            Err(e) => {
                self.opened_database_error(e);
            },
        }
    }

    pub fn activete_closed_database_hood(&mut self) {
        self.database_state = DatabaseState::Closed(ClosedDatabaseAppState::ActiveHood("".to_owned()))
    }
    pub fn deactivete_closed_database_hood(&mut self) {
        self.database_state = DatabaseState::Closed(ClosedDatabaseAppState::None)
    }
    pub fn activete_opened_database_hood(&mut self) {
        self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::ActiveHood("".to_owned()))
    }
    pub fn deactivete_opened_database_hood(&mut self) {
        self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::None)
    }
    pub fn opening_database_error(&mut self, error: String) {
        self.database_state = DatabaseState::Closed(ClosedDatabaseAppState::ActiveHood(error));
        self.clear_buffer();
    }
    pub fn opened_database_error(&mut self, error: String) {
        self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::ActiveHood(error));
        self.clear_buffer();
    }

    pub fn activete_opened_database_active_menu(&mut self) {
        self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::ActiveMenu)
    }
    pub fn activete_opened_database_active_table(&mut self) {
        self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::ActiveTable)
    }

    pub fn release_buffer(&mut self) -> String {
        let result = self.buffer.clone();
        self.buffer.clear();
        result
    }
    pub fn get_buffer(&self) -> String {
        self.buffer.clone()
    }
    pub fn clear_buffer(&mut self) {
        self.buffer.clear()
    }
    pub fn add_char_to_buffer(&mut self, char: char) {
        self.buffer.push(char);
    }
    pub fn remove_last_char_from_the_buffer(&mut self) {
        self.buffer.pop();
    }

    pub fn get_table_list(&self) -> Vec<String> {
        self.database_manager.get_table_list()
    }
    pub fn get_table_count(&self) -> usize {
        self.get_table_list().len()
    }

    pub fn selsect_next_table(&mut self) {
        if let Some(res) = self.selected_table.checked_add(1) {
            if res < self.get_table_count() {
                self.selected_table = res;
            }
        }
    }
    pub fn selsect_priv_table(&mut self) {
        if let Some(res) = self.selected_table.checked_sub(1) {
            self.selected_table = res;
        }
    }
    pub fn get_selected_table_index(&self) -> usize {
        self.selected_table
    }

    pub fn show_table(&mut self) {
        self.displayed_table = self.selected_table
    }

    pub fn get_current_table(&self) -> Result<core::table::Table, String> {
        if self.get_table_count() > 0 {
            Ok(self.database_manager.get_table(&self.get_table_list()[self.displayed_table]).unwrap())
        } else {
            Err("Whoops, no tables in this database :(".to_owned())
        }
    }

    pub fn add_row(&mut self, table_name: String, raw_values: String) {
        let result = self.database_manager.add_row(&table_name, &raw_values);
        match result {
            Ok(_) => {
                self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::None)
            },
            Err(e) => {
                self.opened_database_error(e);
            },
        }
    }
    
    pub fn get_database_name(&self) -> String {
        self.database_manager.get_database_name()
    }
}