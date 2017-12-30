use std::f64;
use uuid::Uuid;
use super::identity::Identity;

pub struct Account {
    id: Uuid,
    balance: f64,
    identity: Identity,
}

impl Account {
    pub fn new(identity: Identity) -> Self {
        Account {
            id: Uuid::new_v4(),
            balance: 0_f64,
            identity,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }

    pub fn withdraw(&mut self, amount: f64) -> TransactionResult {
        if amount <= 0.0_f64 {
            Err(TransactionError::NonPositiveAmount)
        } else if self.balance - amount < 0.0_f64 {
            Err(TransactionError::InsufficientFunds)
        } else {
            self.balance -= amount;
            Ok(self.balance)
        }
    }

    pub fn deposit(&mut self, amount: f64) -> TransactionResult {
        if amount <= 0.0_f64 {
            Err(TransactionError::NonPositiveAmount)
        } else {
            self.balance += amount;
            Ok(self.balance)
        }
    }
}

pub enum TransactionError {
    InsufficientFunds,
    NonPositiveAmount,
}

pub type TransactionResult = Result<f64, TransactionError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut account = Account::default();
        let balance = account.balance();
        assert_eq!(balance, 0.0_f64);

        let balance = account.deposit(10_f64).ok().unwrap();
        assert_eq!(balance, 10_f64);

        let balance = account.withdraw(5_f64).ok().unwrap();
        assert_eq!(balance, 5_f64);
    }
}
