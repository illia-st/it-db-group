use std::net::TcpStream;
use std::ops::Deref;
use std::rc::Rc;
use tungstenite::http::response;
use url::Url;
use tokio::runtime::Runtime;

use tungstenite::client::connect;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket};
use crate::core::{self, table::Table};
use crate::db_api::client_req::ClientRequest;
use crate::db_api::{Decoder, Encoder};
use crate::db_api::db::DatabaseDTO;
use crate::db_api::envelope::Envelope;
use crate::transport::connector::Connector;
use crate::transport::core::{Handler, Receiver, Sender};

use db_manager::Data;
use db_manager::database_manager_client::DatabaseManagerClient;

pub mod db_manager{
    tonic::include_proto!("db");
}

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
    db_manager: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
    should_quit: bool,
    database_state: DatabaseState,
    buffer: String,

    displayed_table: usize,
    selected_table: usize,
    selected_row: usize,
    selected_column: usize,
    sent_req: bool,
}

impl App {
    // TODO: app is going to have a connector where it is going to sent requests
    pub fn new() -> Self {
        Self {
            db_manager: None,
            should_quit: false,
            database_state: DatabaseState::Closed(ClosedDatabaseAppState::None),
            buffer: "".to_string(),
            displayed_table: 0,
            selected_table: 0,
            selected_row: 0,
            selected_column: 0,
            sent_req: false,
        }
    }

    pub fn is_sent_req(&self) -> bool {
        self.sent_req
    }

    pub fn set_connector(&mut self) {
        // self.db_manager = Some(connector);
        let (mut socket, response) = connect(Url::parse("ws://localhost:9091").unwrap()).unwrap();
        self.db_manager = Some(socket);
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

    pub fn update_state_by_server_reply(&mut self) {
        let msg = self.db_manager.as_mut().unwrap().read().unwrap();
        let server_reply= match msg {
            Message::Binary(data) => data,
            _ => {
                log::error!("received wrong message type");
                return;
            }
        };
        self.sent_req = false;
        let envelope = Envelope::decode(server_reply.as_slice());
        match envelope.get_type() {
            "create" => {
                let db_dto = DatabaseDTO::decode(envelope.get_data().to_vec());
                self.database_manager.set_db_dto(db_dto);
                self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::None);
            },
            "delete" => {
                self.database_state = DatabaseState::Closed(ClosedDatabaseAppState::None);
                let _ = self.database_manager.close_db(false);
            },
            "open" => {
                let db_dto = DatabaseDTO::decode(envelope.get_data().to_vec());
                self.database_manager.set_db_dto(db_dto);
                self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::None)
            },
            "close" => {
                self.database_state = DatabaseState::Closed(ClosedDatabaseAppState::None);
                let _ = self.database_manager.close_db(false);
            },
            "error" => {
                let error = String::from_utf8(envelope.get_data().to_vec()).unwrap();
                self.opening_database_error(error);
            }
            _ => panic!("received message type isn't supported"),
        }
    }
    pub fn create_database(&mut self, name: String, database_path: String) {
        let client_req = ClientRequest::new(
            "create".to_string(),
            Some(database_path),
            Some(name),
            None,
        ).encode();
        let envelope  = Envelope::new("client_req", client_req.as_slice()).encode();
        let rt  = Runtime::new().unwrap(); 
        let mut client = rt.block_on(DatabaseManagerClient::connect("https://localhost:7193"));
        let data = tonic::Request::new(
            Data{
                data: envelope
            }
        );
        let response = rt.block_on(client.create_database(data));
        self.sent_req = true;
        // if let Some(connector) = self.db_manager.as_mut() {
        //     connector.send(Message::Binary(envelope)).unwrap();
        //     self.sent_req = true;
        // } else {
        //     self.opening_database_error("couldn't send request to the server, connector isn't set up".to_string());
        // }
    }
    pub fn open_database(&mut self, database_dir_path: String, database_name: String) {
        let client_req = ClientRequest::new(
            "open".to_string(),
            Some(database_dir_path),
            Some(database_name),
            None
        ).encode();
        let envelope  = Envelope::new("client_req", client_req.as_slice()).encode();
        if let Some(connector) = self.db_manager.as_mut() {
            connector.send(Message::Binary(envelope)).unwrap();
            self.sent_req = true;
        } else {
            self.opening_database_error("couldn't send request to the server, connector isn't set up".to_string());
        }
    }
    pub fn close_database(&mut self, need_to_save: bool) {
        let db_to_save = if need_to_save {
            Some(self.database_manager.get_db().unwrap())
        } else {
            None
        };
        let client_req = ClientRequest::new(
            "close".to_string(),
            None,
            None,
            db_to_save
        ).encode();
        let envelope  = Envelope::new("client_req", client_req.as_slice()).encode();
        if let Some(connector) = self.db_manager.as_mut() {
            connector.send(Message::Binary(envelope)).unwrap();
            self.sent_req = true;
        } else {
            self.opening_database_error("couldn't send request to the server, connector isn't set up".to_string());
        }
    }
    pub fn delete_database(&mut self, database_dir_path: String, database_name: String) {
        let client_req = ClientRequest::new(
            "delete".to_string(),
            Some(database_dir_path),
            Some(database_name),
            None
        ).encode();
        let envelope  = Envelope::new("client_req", client_req.as_slice()).encode();
        if let Some(connector) = self.db_manager.as_mut() {
            connector.send(Message::Binary(envelope)).unwrap();
            self.sent_req = true;
        } else {
            self.opening_database_error("couldn't send request to the server, connector isn't set up".to_string());
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
    pub fn delete_row(&mut self, table_name: String, raw_index_value: String) {
        let parsing_result = raw_index_value.parse::<u64>();
        match &parsing_result {
            Ok(_) => {
                self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::None)
            },
            Err(e) => {
                self.opened_database_error(e.to_string());
            },
        }

        let result = self.database_manager.delete_row(&table_name, parsing_result.unwrap());
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

    pub fn delete_table(&mut self, table_name: String) {
        let result = self.database_manager.delete_table(&table_name);
        match result {
            Ok(_) => {
                self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::None)
            },
            Err(e) => {
                self.opened_database_error(e);
            },
        }
    }

    pub fn rename_row(&mut self, table_name: String, columns: String) {
        let column_names = columns.split_terminator(';').collect::<Vec<&str>>().iter().map(|s| s.to_owned().to_owned()).collect();
        let result = self.database_manager.rename(&table_name, column_names);
        match result {
            Ok(_) => {
                self.database_state = DatabaseState::Opened(OpenedDatabaseAppState::None)
            },
            Err(e) => {
                self.opened_database_error(e);
            },
        }
    }


}