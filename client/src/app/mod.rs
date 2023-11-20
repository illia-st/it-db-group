use std::ops::Deref;
use std::rc::Rc;

use db_manager::db_manager::DatabaseManager;
use core::{self, table::Table};
use transport::connectors::connector::Connector;
use transport::connectors::core::Sender;

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

pub struct App {
    db_manager: Rc<dyn Sender>,
    should_quit: bool,
    database_state: DatabaseState,
    buffer: String,
    database_manager: DatabaseManager,

    displayed_table: usize,
    selected_table: usize,
    selected_row: usize,
    selected_column: usize,
}

impl App {
    // TODO: app is going to have a connector where it is going to sent requests
    pub fn new(db_manager: Rc<dyn Sender>) -> Self {
        Self {
            db_manager,
            should_quit: false,
            database_state: DatabaseState::Closed(ClosedDatabaseAppState::None),
            buffer: "".to_string(),
            database_manager: Default::default(),
            displayed_table: 0,
            selected_table: 0,
            selected_row: 0,
            selected_column: 0,
        }
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
        let data = format!("{name} {database_path}");
        self.db_manager.send(data.as_bytes());
        // let result = self.database_manager.create_db(&name, &database_path);

        // match result {
        //     Ok(_) => {
        //         self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::None)
        //     },
        //     Err(e) => {
        //         self.opening_database_error(e);
        //     },
        // }
    }
    pub fn open_database(&mut self, database_dir_path: String, database_name: String) {
        let data = format!("{database_dir_path} {database_name}");
        self.db_manager.send(data.as_bytes());
        // let result = self.database_manager.read_db_from_directory(&database_dir_path, &database_name);
        // match result {
        //     Ok(_) => {
        //         self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::None)
        //     },
        //     Err(e) => {
        //         self.opening_database_error(e);
        //     },
        // }
    }
    pub fn close_database(&mut self, need_to_save: bool) {
        let data = format!("{need_to_save}");
        self.db_manager.send(data.as_bytes());
        // let result = self.database_manager.close_db(need_to_save);
        // match result {
        //     Ok(_) => {
        //         self.database_state = DatabaseState::Closed(ClosedDatabaseAppState::None)
        //     },
        //     Err(e) => {
        //         self.opening_database_error(e);
        //     },
        // }
    }
    pub fn delete_database(&mut self, database_dir_path: String, database_name: String) {
        let data = format!("{database_dir_path} {database_name}");
        self.db_manager.send(data.as_bytes());
        // let result = self.database_manager.delete_db(&database_dir_path, &database_name);
        // match result {
        //     Ok(_) => {
        //         self.database_state = DatabaseState::Closed(ClosedDatabaseAppState::None)
        //     },
        //     Err(e) => {
        //         self.opening_database_error(e);
        //     },
        // }
    }

    pub fn create_table(&mut self, table_name: String, columns: String, data_types: String) {
        let column_names = columns.split_terminator(';').collect::<Vec<&str>>();
        let column_data = data_types.split_terminator(';').collect::<Vec<&str>>();

        let data = format!("{table_name} {columns} {data_types}");
        self.db_manager.send(data.as_bytes());

        // let result = self.database_manager.create_table(
        //     table_name.deref(),
        //     column_names,
        //     column_data
        // );
        // match result {
        //     Ok(_) => {
        //         self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::None)
        //     },
        //     Err(e) => {
        //         self.opened_database_error(e);
        //     },
        // }
    }

    pub fn activete_closed_database_hood(&mut self) {
        self.database_state = DatabaseState::Closed(ClosedDatabaseAppState::ActiveHood("".to_owned()))
    }
    pub fn deactivete_closed_database_hood(&mut self) {
        self.database_state = DatabaseState::Closed(ClosedDatabaseAppState::None)
    }
    pub fn activete_opened_database_hood(&mut self) {
        self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::ActiveHood("".to_owned()));
        self.reset_column();
        self.reset_row();
    }
    pub fn deactivete_opened_database_hood(&mut self) {
        self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::None);
        self.reset_column();
        self.reset_row();
    }
    pub fn opening_database_error(&mut self, error: String) {
        self.database_state = DatabaseState::Closed(ClosedDatabaseAppState::ActiveHood(error));
        self.clear_buffer();
    }
    pub fn opened_database_error(&mut self, error: String) {
        self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::ActiveHood(error));
        self.reset_column();
        self.reset_row();
        self.clear_buffer();
    }

    pub fn activete_opened_database_active_menu(&mut self) {
        self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::ActiveMenu);
        self.reset_column();
        self.reset_row();
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

    ///////////////////////////
    //START SELECTION SECTION//
    ///////////////////////////
    
    //Selected table
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

    //Selected cell row
    pub fn selsect_next_row(&mut self) {
        if let Some(res) = self.selected_row.checked_add(1) {
            if res < self.get_current_table().unwrap().get_rows().len() {
                self.selected_row = res;
            }
        }
    }
    pub fn selsect_priv_row(&mut self) {
        if let Some(res) = self.selected_row.checked_sub(1) {
            self.selected_row = res;
        }
    }
    pub fn reset_row(&mut self) {
        self.selected_row = 0;
    }
    pub fn get_selected_row_index(&self) -> usize {
        self.selected_row
    }

    //Selected cell column
    pub fn selsect_next_column(&mut self) {
        if let Some(res) = self.selected_column.checked_add(1) {
            if res < self.get_current_table().unwrap().get_columns().len() {
                self.selected_column = res;
            }
        }
    }
    pub fn selsect_priv_column(&mut self) {
        if let Some(res) = self.selected_column.checked_sub(1) {
            self.selected_column = res;
        }
    }
    pub fn reset_column(&mut self) {
        self.selected_column = 0;
    }
    pub fn get_selected_column_index(&self) -> usize {
        self.selected_column
    }

    /////////////////////////
    //END SELECTION SECTION//
    /////////////////////////


    pub fn get_current_table(&self) -> Result<core::table::Table, String> {
        if self.get_table_count() > 0 {
            Ok(self.database_manager.get_table(&self.get_table_list()[self.displayed_table]).unwrap())
        } else {
            Err("Whoops, no tables in this database :(".to_owned())
        }
    }

    pub fn add_row(&mut self, table_name: String, raw_values: String) {
        let data = format!("{table_name} {raw_values}");
        self.db_manager.send(data.as_bytes());
        // let result = self.database_manager.add_row(&table_name, &raw_values);
        // match result {
        //     Ok(_) => {
        //         self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::None)
        //     },
        //     Err(e) => {
        //         self.opened_database_error(e);
        //     },
        // }
    }
    pub fn delete_row(&mut self, table_name: String, raw_index_value: String) {
        let parsing_result = raw_index_value.parse::<u64>();
        let data = format!("delete {table_name} {raw_index_value}");
        self.db_manager.send(data.as_bytes());
        // match &parsing_result {
        //     Ok(_) => {
        //         self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::None)
        //     },
        //     Err(e) => {
        //         self.opened_database_error(e.to_string());
        //     },
        // }
        //
        // let result = self.database_manager.delete_row(&table_name, parsing_result.unwrap());
        // match result {
        //     Ok(_) => {
        //         self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::None)
        //     },
        //     Err(e) => {
        //         self.opened_database_error(e);
        //     },
        // }
    }
    
    pub fn get_database_name(&self) -> String {
        let data = "get db name".to_string();
        self.db_manager.send(data.as_bytes());
        "".to_string()
    }

    pub fn delete_table(&mut self, table_name: String) {
        let data = format!("delete table {table_name} ");
        self.db_manager.send(data.as_bytes());
        // let result = self.database_manager.delete_table(&table_name);
        // match result {
        //     Ok(_) => {
        //         self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::None)
        //     },
        //     Err(e) => {
        //         self.opened_database_error(e);
        //     },
        // }
    }

    pub fn rename_row(&mut self, table_name: String, columns: String) {
        let data = format!("rename row {table_name} {columns}");
        self.db_manager.send(data.as_bytes());
        // let column_names = columns.split_terminator(';').collect::<Vec<&str>>().iter().map(|s| s.to_owned().to_owned()).collect();
        // let result = self.database_manager.rename(&table_name, column_names);
        // match result {
        //     Ok(_) => {
        //         self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::None)
        //     },
        //     Err(e) => {
        //         self.opened_database_error(e);
        //     },
        // }
    }
}