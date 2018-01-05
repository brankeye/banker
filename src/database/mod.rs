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
            let mut directory: String = "data/".to_owned();
            directory.push_str(name);
            if let Err(e) = fs::create_dir_all(&directory) {
                println!("Failed to make dir. {:?}", e);
            }
            let store = Rc::new(Store::new(directory.to_owned()));
            self.stores.entry(name.to_string()).or_insert(store);
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

    // 1. Validate input key.
    // 2. Open file at path.
    // 3. Read file as json.
    // 4. Deserialize json to model.
    pub fn get(&mut self, key: &str) -> DbResult<T> {
        let mut data = String::new();
        match File::open(self.path(key)) {
            Ok(mut file) => {
                if let Err(err) = file.read_to_string(&mut data) {
                    return Err(DbError::BadFile); 
                }
                let json = data.as_str();
                let model: T = Table::deserialize(&json).ok().unwrap();
                Ok(model)
            },
            Err(err) => Err(DbError::FileDoesNotExist),
        }
    }

    pub fn add(&mut self, key: &str, value: &T) -> DbResult<()> {
        let serialized = Table::serialize(value).ok().unwrap();
        let mut file = File::create(self.path(key)).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
        Ok(())
    }

    pub fn delete(&mut self, key: &str) -> DbResult<()> {
        fs::remove_file(self.path(key));
        Ok(())
    }

    fn path(&self, key: &str) -> String {
        let mut path = self.directory.clone();
        path.push_str(&"/");
        path.push_str(key);
        path.push_str(&".json");
        path
    } 

    fn serialize(model: &T) -> Result<String, DbError> {
        serde_json::to_string(&model).map_err(|_| DbError::BadFile)
    }

    fn deserialize(model: &str) -> Result<T, DbError> {
        serde_json::from_str::<T>(model).map_err(|_| DbError::BadFile)
    }
}

pub enum DbError {
    FileDoesNotExist,
    BadFile,
    TableDoesNotExist,
}

pub type DbResult<T> = Result<T, DbError>;

