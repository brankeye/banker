use std::marker::PhantomData;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::collections::HashMap;
use std::rc::Rc;
use serde_json;
use serde::ser::Serialize;
use serde::de::DeserializeOwned;

pub struct Database {
    stores: HashMap<String, Rc<Store>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            stores: HashMap::new(),
        }
    }

    pub fn table<T>(&mut self) -> Table<T> where T : DbModel {
        let t = T::name();
        let name = t.as_str();
        if !self.stores.contains_key(name) {
            let mut owned_string: String = "data/".to_owned();
            owned_string.push_str(name);
            if let Err(e) = fs::create_dir_all(owned_string) {
                println!("Failed to make dir. {:?}", e);
            }
            let store = Store::new(name.to_owned());
            self.stores.entry(name.to_string()).or_insert(Rc::new(store));
        }
        self.stores.get(name).unwrap().as_table::<T>()
    }
}

pub trait DbModel: Serialize + DeserializeOwned {
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
    
    pub fn as_table<T>(&self) -> Table<T> where T: DbModel {
        Table::new(self.directory.clone())
    }
}

pub struct Table<T> {
   directory: String,
   _phantom : PhantomData<T>,
}

impl<T: DbModel> Table<T> {
    fn new(directory: String) -> Self {
        Table {
            directory,
            _phantom: PhantomData,
        }
    }

    pub fn get(&mut self, key: &str) -> DbResult<T> {
        let mut data = String::new();
        let mut path = "data/".to_owned();
        path.push_str(T::name().as_str());
        path.push_str(&"/");
        path.push_str(key);
        let mut f = File::open(path).expect("Unable to open file");
        f.read_to_string(&mut data).expect("Unable to read string");
        let data_as_str = data.as_str();
        let model: T = Table::deserialize(&data_as_str);
        Ok(model)
    }

    pub fn add(&mut self, key: &str, value: &T) -> DbResult<()> {
        let serialized = Table::serialize(value);
        let mut path = "data/".to_owned();
        path.push_str(T::name().as_str());
        path.push_str(&"/");
        path.push_str(key);
        let mut file = File::create(path).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
        Ok(())
    }

    pub fn delete(&mut self, key: &str) -> DbResult<()> {
        let mut path = "data/".to_owned();
        path.push_str(T::name().as_str());
        path.push_str(&"/");
        path.push_str(key);
        fs::remove_file(path);
        Ok(())
    }

    fn serialize(model: &T) -> String {
        serde_json::to_string(&model).unwrap().to_owned()
    }

    fn deserialize(model: &str) -> T {
        let deserialized: T = serde_json::from_str(model).unwrap();
        deserialized
    }
}

pub enum DbError {
    TableDoesNotExist,
}

pub type DbResult<T> = Result<T, DbError>;

