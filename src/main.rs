extern crate banker;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use banker::identity::Identity;
use banker::bank::Bank;
use banker::account::Account;
use banker::database::Database;
use banker::database::Table;

struct Db {
    database: Database,
}

impl Db {
    pub fn new() -> Self {
        Db {
            database: Database::new()
        }
    }

    pub fn accounts(&mut self) -> Table<Account> {
        self.database.table::<Account>()
    }
}

fn main() {
        let identity = Identity::new(
            String::from("Jeff"), 
            String::from("Hanson"), 
            47, 
            String::from("123 Main St.")
        );

        let mut tangerine = Bank::new(String::from("Tangerine"));
        let account = Account::new(identity);
        tangerine.add_account(account).ok().unwrap();
        tangerine.list_accounts();

        let mut db = Db::new();
        let mut accounts = db.accounts();
 
        let identity = Identity::new(
            String::from("Jeff"), 
            String::from("Hanson"), 
            47, 
            String::from("123 Main St.")
        );
        let account = Account::new(identity);
        let id: &str = &account.id().hyphenated().to_string()[..];
        accounts.add(id, &account);
        let acct: Account = accounts.get(id).ok().unwrap();
        println!("The jazz: {:?}", acct);

        //accounts.delete(id).ok().unwrap();
}
