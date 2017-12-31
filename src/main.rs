extern crate banker;

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
        //accounts.add("1", "test").ok().unwrap();
        //let test = accounts.get("1").ok().unwrap();
        //println!("Should say test: {}", test);
}
