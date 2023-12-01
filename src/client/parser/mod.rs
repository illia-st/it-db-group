use clap::{Arg, ArgAction, Command};

pub fn get_parser() -> Command {
    Command::new("database")
            .subcommands([
                Command::new("create")
                    .args([
                        Arg::new("database")
                            .short('d')
                            .conflicts_with("table")
                            .required_unless_present("table")
                            .action(ArgAction::SetTrue),
                        Arg::new("table")
                            .short('t')
                            .conflicts_with("database")
                            .required_unless_present("database")
                            .action(ArgAction::SetTrue),

                        Arg::new("name")
                            .short('n')
                            .required(true)
                            .action(ArgAction::Set),
                        
                        Arg::new("database_path")
                            .short('p')
                            .conflicts_with("table")
                            .required_unless_present("table")
                            .action(ArgAction::Set),

                        Arg::new("table_column_names")
                            .short('c')
                            .conflicts_with("database")
                            .required_unless_present("database")
                            .action(ArgAction::Set),
                        Arg::new("table_types")
                            .short('v')
                            .conflicts_with("database")
                            .required_unless_present("database")
                            .action(ArgAction::Set),
                    ]),

                Command::new("delete")
                    .args([
                        Arg::new("database")
                            .short('d')
                            .conflicts_with("table")
                            .required_unless_present("table")
                            .action(ArgAction::SetTrue),
                        Arg::new("table")
                            .short('t')
                            .conflicts_with("database")
                            .required_unless_present("database")
                            .action(ArgAction::SetTrue),

                        Arg::new("name")
                            .short('n')
                            .required(true)
                            .action(ArgAction::Set),
                        
                        Arg::new("database_path")
                            .short('p')
                            .conflicts_with("table")
                            .required_unless_present("table")
                            .action(ArgAction::Set),
                    ]),

                Command::new("open")
                    .args([
                        Arg::new("database_path")
                            .short('p')
                            .required(true)
                            .action(ArgAction::Set),
                        Arg::new("database_name")
                            .short('n')
                            .required(true)
                            .action(ArgAction::Set),
                    ]),

                Command::new("close")
                    .args([
                        Arg::new("save")
                            .short('s')
                            .required(false)
                            .action(ArgAction::SetTrue)
                    ]),

                Command::new("add")
                    .args([
                        Arg::new("table_name")
                            .short('n')
                            .required(true)
                            .action(ArgAction::Set),
                        Arg::new("row_value")
                            .short('r')
                            .required(true)
                            .action(ArgAction::Set)
                    ]),
                
                Command::new("remove")
                    .args([
                        Arg::new("table_name")
                            .short('n')
                            .required(true)
                            .action(ArgAction::Set),
                        Arg::new("row_index")
                            .short('i')
                            .required(true)
                            .action(ArgAction::Set),
                    ]),

                Command::new("rename")
                    .args([
                        Arg::new("table_name")
                            .short('n')
                            .required(true)
                            .action(ArgAction::Set),
                        Arg::new("table_column_names")
                            .short('c')
                            .required(true)
                            .action(ArgAction::Set),
                    ]),

                Command::new("join")
                    .args([
                        Arg::new("left_table_name")
                            .short('l')
                            .required(true)
                            .action(ArgAction::Set),
                        Arg::new("right_table_name")
                            .short('r')
                            .required(true)
                            .action(ArgAction::Set),
                            
                        Arg::new("column_name")
                            .short('c')
                            .required(true)
                            .action(ArgAction::Set),
                    ]),
            ])
}

#[cfg(test)]
mod tests {
    use super::get_parser;

    #[test]
    fn parser_is_parsing_correctly() {
        let mut command = get_parser();
        
        let args = vec!["database", "create", "-t"];
        assert!(command.try_get_matches_from_mut(&args).is_err());
        let args = vec!["database", "create", "-t", "-n", "\"\"", "-c", "\"\"", "-v", "\"\""];
        assert!(command.try_get_matches_from_mut(&args).is_ok());
        let args = vec!["database", "create", "-d", "-n", "\"\"", "-p", "\"\""];
        assert!(command.try_get_matches_from_mut(&args).is_ok());
        let args = vec!["database", "close"];
        assert!(command.try_get_matches_from_mut(&args).is_ok());
        match command.try_get_matches_from_mut(args).unwrap().subcommand() {
            Some(("close", arg)) => {
                assert!(!arg.get_flag("save"))
            },
            _ => todo!(),
        }
        let args = vec!["database", "close", "-s"];
        assert!(command.try_get_matches_from_mut(&args).is_ok());
        let args = vec!["database", "close", "-s", "\"\""];
        assert!(command.try_get_matches_from_mut(&args).is_err());
    }   
}