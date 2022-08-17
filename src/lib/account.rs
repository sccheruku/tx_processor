use std::ops::{AddAssign, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Account {
    // client, available, held, total, locked
    pub client: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}

impl Account {
    pub fn new(client: u16) -> Self {
        return Account {
            client,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        };
    }

    pub fn deposit(&mut self, amount: f64) {
        // account locked, we do not process this transaction
        if self.locked {
            return;
        }
        
        /*
            Note: Overflows are worth watching out for.
            But, I think, this example is dealing with CreditCard transactions, 
            It's unlikely someone has the funds to overflow a f64. 
            Even if they do, the CreditCard companies typically have an upper limit
            which is not even close to f64
        */
        self.available.add_assign(amount);
        self.total.add_assign(amount);
    }

    pub fn withdraw(&mut self, amount: f64) {
        if self.locked {
            return;
        }
        if self.available >= amount {
            self.available.sub_assign(amount);
            self.total.sub_assign(amount);
        }
        // not enough funds
    }

    pub fn dispute(&mut self, amount: f64) {
        if self.locked {
            return;
        }
        self.available.sub_assign(amount);
        self.held.add_assign(amount);
        // TODO: At this point, what if the available amount is negative??
    }

    pub fn resolve(&mut self, amount: f64) {
        if self.locked {
            return;
        }
        // TODO: held must be greater than or equal to amount  ??
        // TODO: do we handle disputed transactions for deposits AND withdrawals ?
        self.available.add_assign(amount);
        self.held.sub_assign(amount);
    }

    pub fn chargeback(&mut self, amount: f64) {
        if self.locked {
            return;
        }
        self.locked = true;
        self.total.sub_assign(amount);
        self.held.sub_assign(amount);
    }
}

#[cfg(test)]
mod tests {
    use super::Account;

    #[test]
    fn should_increment_funds_after_deposit() {
        let mut account = Account::new(100);
        let amount = 250.0;
        account.deposit(amount);
        assert!(account.available == amount);
        assert!(account.held == 0.0);
        assert!(account.total == amount);
    }

    #[test]
    fn withdraw_should_fail_if_insufficient_funds() {
        let mut account = Account::new(100);
        let amount = 250.0;
        account.deposit(100.0);
        account.withdraw(amount);
        assert!(account.available == 100.0);
        assert!(account.total == 100.0);
    }

    #[test]
    fn withdraw_should_succeed_if_sufficient_funds() {
        let mut account = Account::new(100);
        let amount = 250.0;
        account.deposit(1000.0);
        account.withdraw(amount);
        assert!(account.available == (1000.0 - amount));
        assert!(account.total == (1000.0 - amount));
    }

    #[test]
    fn dispute_should_increase_held_and_decrease_available_funds() {
        let mut account = Account::new(100);
        let amount = 250.0;
        let total = 1000.0;
        account.deposit(total);
        account.dispute(amount);

        assert!(account.available == total - amount);
        assert!(account.total == total);
        assert!(account.held == amount);
        assert!(account.total == account.available + account.held);
    }

    #[test]
    fn resolve_should_decrease_held_and_increase_available_funds() {
        let mut account = Account::new(100);
        let disputed_amount = 250.0;
        let resolved_amount = 50.0;
        let total = 1000.0;

        account.deposit(total);
        account.dispute(disputed_amount);
        account.resolve(resolved_amount);

        assert!(account.held == disputed_amount - resolved_amount);
        assert!(account.available == total - disputed_amount + resolved_amount);

        assert!(account.total == total);
        assert!(account.total == account.available + account.held);
    }

    #[test]
    pub fn chargeback_should_freeze_account() {
        let mut account = Account::new(100);
        let total = 1000.0;
        let chargeback_amount = 250.0;
        account.deposit(total);

        account.chargeback(chargeback_amount);

        assert!(account.locked);
    }

    #[test]
    pub fn chargeback_should_decrease_held_and_total_funds() {
        let mut account = Account::new(100);
        let total = 1000.0;
        let chargeback_amount = 250.0;
        account.deposit(total);
        account.dispute(chargeback_amount);

        account.chargeback(chargeback_amount);

        assert!(account.total == total - chargeback_amount);
        assert!(account.held == 0.0);
    }

    #[test]
    pub fn deposit_is_ignored_if_account_is_locked() {
        let mut account = Account::new(100);
        account.locked = true;
        account.deposit(100.0);

        assert!(account.held == 0.0);
        assert!(account.total == 0.0);
        assert!(account.available == 0.0);
    }

    #[test]
    pub fn withdraw_is_ignored_if_account_is_locked() {
        let mut account = Account::new(100);
        let amount = 100.0;
        account.deposit(amount);

        account.locked = true;
        account.withdraw(amount);
        assert!(account.held == 0.0);
        assert!(account.total == amount);
        assert!(account.available == amount);
    }

    #[test]
    pub fn dispute_is_ignored_if_account_is_locked() {
        let mut account = Account::new(100);
        let amount = 100.0;
        account.deposit(amount);

        account.locked = true;
        account.dispute(amount);
        assert!(account.held == 0.0);
        assert!(account.total == amount);
        assert!(account.available == amount);
    }

    #[test]
    pub fn resolve_is_ignored_if_account_is_locked() {
        let mut account = Account::new(100);
        let amount = 100.0;
        account.deposit(amount);
        account.dispute(amount);

        account.locked = true;
        account.resolve(amount);
        assert!(account.held == amount);
        assert!(account.total == amount);
        assert!(account.available == 0.0);
    }

    #[test]
    pub fn chargeback_is_ignored_if_account_is_locked() {
        let mut account = Account::new(100);
        let amount = 100.0;
        account.deposit(amount);
        account.dispute(amount);

        account.locked = true;
        account.chargeback(amount);
        assert!(account.held == amount);
        assert!(account.total == amount);
        assert!(account.available == 0.0);
    }
}
