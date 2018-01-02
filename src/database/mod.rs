extern crate serde;
extern crate serde_json;

use std::fs;
use std::env;
use std::collections::HashMap;
use std::rc::Rc;
use serde::ser::Serialize;
use serde::de::Deserialize;

pub struct Database {
    stores: HashMap<String, Rc<Store>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            stores: HashMap::new(),
        }
    }

    pub fn table<'a, T>(&mut self) -> Table<T> where T : DbModel<'a> {
        let t = T::name();
        let name = t.as_str();
        if !self.stores.contains_key(name) {
            let mut owned_string: String = "data/".to_owned();
            owned_string.push_str(name);
            match fs::create_dir_all(owned_string) {
                Ok(_) => {},
                Err(e) => { println!("Failed to make dir. {:?}", e); },
            }
            let store = Store::new(name.to_owned());
            self.stores.entry(name.to_string()).or_insert(Rc::new(store));
        }
        self.stores.get(name).unwrap().as_table::<T>()
    }
}

pub trait DbModel<'a>: Serialize + Deserialize<'a> {
    fn name() -> String;
}

pub struct Store {
    directory: String,    
}

impl Store {
    pub fn new(directory: String) -> Self {
        Store {
            directory,
        }
    }
    
    pub fn as_table<T>(&self) -> Table<T> {
        Table::new(self.directory.clone())
    }
}

pub struct Table<T> {
   directory: String,
   data: Option<T>,
}

impl<T> Table<T> {
    fn new(directory: String) -> Self {
        Table {
            directory,
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

