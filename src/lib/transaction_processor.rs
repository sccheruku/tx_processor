use super::{account::Account, transaction::Transaction, transaction::TransactionType};
use std::collections::HashMap;

pub struct TransactionProcessor {
    accounts: HashMap<u16, Account>,
    transactions: HashMap<u32, Transaction>,
}

impl TransactionProcessor {
    pub fn new() -> Self {
        return TransactionProcessor {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
        };
    }

    fn get_or_create_account(&mut self, client: u16) -> Account {
        let account_opt = self.accounts.get(&client);
        match account_opt {
            Some(account) => *account,
            None => {
                let account = Account::new(client);
                self.accounts.insert(client, account);
                return account;
            }
        }
    }

    pub fn process_transaction(&mut self, transaction: Transaction) {
        match transaction.transaction_type {
            TransactionType::Deposit | TransactionType::Withdrawal => {
                self.transactions.insert(transaction.tx, transaction);
            }
            _ => {}
        }

        match transaction.transaction_type {
            TransactionType::Deposit => self.process_deposit(transaction),
            TransactionType::Withdrawal => self.process_withdrawal(transaction),
            TransactionType::Dispute => self.process_dispute(transaction),
            TransactionType::Resolve => self.process_resolve(transaction),
            TransactionType::Chargeback => self.process_chargeback(transaction),
        }
    }

    fn process_deposit(&mut self, transaction: Transaction) {
        let mut account = self.get_or_create_account(transaction.client);
        account.deposit(transaction.amount.unwrap());
        self.accounts.insert(transaction.client, account);
    }
    fn process_withdrawal(&mut self, transaction: Transaction) {
        let mut account = self.get_or_create_account(transaction.client);
        account.withdraw(transaction.amount.unwrap());
        self.accounts.insert(transaction.client, account);
    }
    fn process_dispute(&mut self, tx: Transaction) {
        let mut account = self.get_or_create_account(tx.client);

        if self.transactions.contains_key(&tx.tx){
            let mut disputed_tx = self.get_transaction(&tx.tx);
            if tx.client == disputed_tx.client {
                account.dispute(disputed_tx.amount.unwrap());
                self.accounts.insert(tx.client, account);
                
                disputed_tx.set_disputed();
                self.transactions.insert(disputed_tx.tx, disputed_tx);
            }
        }
    }
    fn process_resolve(&mut self, tx: Transaction) {

        let mut account = self.get_or_create_account(tx.client);

        if self.transactions.contains_key(&tx.tx){
            let mut disputed_tx = self.get_transaction(&tx.tx);
            if tx.client == disputed_tx.client {
                account.resolve(disputed_tx.amount.unwrap());
                self.accounts.insert(tx.client, account);
                
                disputed_tx.remove_disputed();
                self.transactions.insert(disputed_tx.tx, disputed_tx);
            }
        }
    }
    fn process_chargeback(&mut self, tx: Transaction) {
        let mut account = self.get_or_create_account(tx.client);
        let disputed_transaction_opt = self.transactions.get(&tx.tx);
        match disputed_transaction_opt {
            Some(disputed_tx) => {
                if !disputed_tx.disputed {
                    return;
                }
                if tx.client == disputed_tx.client {
                    account.chargeback(disputed_tx.amount.unwrap());
                    self.accounts.insert(tx.client, account);
                }
            }
            None => {}
        }
    }

    pub fn get_account_states(self) -> Vec<Account> {
        let accounts: Vec<Account> = self.accounts.values().cloned().collect();
        accounts
    }

    pub fn get_transaction(&mut self, tx: &u32) -> Transaction {
        return *self.transactions.get(tx).unwrap();
    }

}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::lib::{account::Account, transaction::{Transaction, TransactionType}};
    use super::TransactionProcessor;

    #[test]
    fn should_create_new_account() {
        let mut processor = TransactionProcessor::new();
        assert!(processor.accounts.len() == 0);
        processor.get_or_create_account(100);
        assert!(processor.accounts.len() == 1);
    }

    #[test]
    fn should_return_existing_account() {
        let mut accounts: HashMap<u16, Account> = HashMap::new();
        let mut account = Account::new(100);
        account.available = 5.0;
        accounts.insert(100, account);
        let mut processor = TransactionProcessor {
            accounts,
            transactions: HashMap::new(),
        };
        let returned_account = processor.get_or_create_account(100);
        assert!(returned_account.client == 100);
        assert!(returned_account.available == 5.0);
    }

    #[test]
    pub fn process_transaction_should_add_to_transactions(){
        let mut processor = TransactionProcessor {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
        };
        processor.process_transaction(Transaction {
            transaction_type: TransactionType::Deposit,
            client: 7000,
            tx: 1,
            amount: Some(1.0),
            disputed: false
        });
        assert!(processor.transactions.len() > 0);
    }

    #[test]
    pub fn process_deposit_should_call_account_deposit(){
        let mut processor = TransactionProcessor {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
        };
        processor.process_transaction(Transaction {
            transaction_type: TransactionType::Deposit,
            client: 7000,
            tx: 1,
            amount: Some(1.0),
            disputed: false
        });
        assert!(processor.accounts.len() > 0);
        let account = processor.accounts.get(&7000).unwrap();
        assert!(account.available == 1.0);
    }

    #[test]
    pub fn process_withdraw_should_call_account_deposit(){
        let mut accounts: HashMap<u16, Account> = HashMap::new();
        let client_id = 7000;
        let mut account = Account::new(client_id);
        account.deposit(100.0);
        accounts.insert(client_id, account);
        let mut processor = TransactionProcessor {
            accounts,
            transactions: HashMap::new(),
        };
        processor.process_transaction(Transaction {
            transaction_type: TransactionType::Withdrawal,
            client: client_id,
            tx: 1,
            amount: Some(50.0),
            disputed: false
        });
        assert!(processor.accounts.len() > 0);
        let account = processor.accounts.get(&client_id).unwrap();
        assert!(account.available == 50.0);
    }

    // Additional Tests to consider
    /*
        process_dispute_should_ignore_transaction_if_it_is_not_related_to_the_same_client
        process_dispute_should_call_account_dispute

        process_resolve_should_ignore_transaction_if_it_is_not_related_to_the_same_client
        process_resolve_should_call_account_resolve
        process_resolve_should_ignore_transaction_if_it_is_not_disputed

        process_chargeback_should_ignore_transaction_if_it_is_not_related_to_the_same_client
        process_chargeback_should_call_account_chargeback
        process_chargeback_should_ignore_transaction_if_it_is_not_disputed
    */


}
