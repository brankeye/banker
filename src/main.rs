extern crate banker;

use banker::identity::Identity;
use banker::bank::Bank;
use banker::account::Account;

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
}
