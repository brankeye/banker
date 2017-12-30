use std::fs;
use std::env;
use std::collections::HashMap;

pub struct Database {
    connectors: HashMap<String, Connector>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            connectors: HashMap::new(),
        }
    }

    pub fn table<T>(&mut self, name: &str) -> Table<T> {
        if !self.connectors.contains_key(name) {
            fs::create_dir("/".to_string() + name).ok();
            let connector = Connector::new(name);
            self.connectors.entry(name.to_string()).or_insert(connector);
        }
        let connector = self.connectors.get(name).unwrap();
        Table::new(&connector)
    }
}

struct Connector {
    directory: String,
}

impl Connector {
    pub fn new(name: &str) -> Self {
        Connector {
            directory: name.to_string(),
        }
    }
}

pub struct Table<T> {
   directory: String,
   data: Option<T>,
}

impl<T> Table<T> {
    fn new(connector: &Connector) -> Self {
        Table {
            directory: connector.directory.clone(),
            data: None,
        }
    }

    pub fn get(&mut self, key: &str) -> DbResult<T> {
        unimplemented!()
    }

    pub fn add(&mut self, key: &str, value: T) -> DbResult<()> {
        unimplemented!()
    }

    pub fn update(&mut self, key: &str, value: T) -> DbResult<()> {
        unimplemented!()
    }

    pub fn delete(&mut self, key: &str) -> DbResult<()> {
        unimplemented!()
    }  
}

pub enum DbError {
    TableDoesNotExist,
}

pub type DbResult<T> = Result<T, DbError>;
