extern crate banker;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use banker::identity::Identity;
use banker::bank::Bank;
use banker::account::Account;
use banker::database::Database;

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

        let mut db = Database::new();
        let mut accounts = db.table::<Account>();
 
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

        accounts.delete(id).ok().unwrap();
}
