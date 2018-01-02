use uuid::Uuid;
use std::collections::HashMap;
use super::account::Account;
use std::rc::Rc;

pub struct Bank {
    id: Uuid, 
    name: String,
    accounts: HashMap<Uuid, Account>,
}

impl<'a> Bank {
    pub fn new(name: String) -> Self {
        Bank {
            id: Uuid::new_v4(),
            name,
            accounts: HashMap::new(),
        }
    }

    pub fn add_account(&mut self, account: Account) -> ServiceResult {
        if self.accounts.contains_key(&account.id()) {
            Err(ServiceError::AccountAlreadyExists)
        } else {
            self.accounts.entry(account.id()).or_insert(account);
            Ok(())
        }
    }

    pub fn remove_account(&mut self, account_id: Uuid) -> ServiceResult {
        if self.accounts.contains_key(&account_id) {
            self.accounts.remove(&account_id);
            Ok(())
        } else {
            Err(ServiceError::AccountDoesNotExist)
        }
    }

    pub fn list_accounts(&mut self) {
        for (id, ref account) in self.accounts.iter() {
            println!("Account Id {}", id);
        } 
    }
}

pub enum ServiceError {
    AccountAlreadyExists,
    AccountDoesNotExist,
}

pub type ServiceResult = Result<(), ServiceError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut tangerine = Bank::new(String::from("Tangerine"));
        let account = Account::default();
        let account_id = account.id();
        assert_eq!(tangerine.add_account(account).is_ok(), true);
        assert_eq!(tangerine.remove_account(account_id).is_ok(), true);
    }
}
