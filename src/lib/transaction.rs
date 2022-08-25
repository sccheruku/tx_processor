use std::{fmt::Error, num::ParseIntError};
use csv::StringRecord;

#[derive(Debug, Clone, Copy)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

impl TransactionType {
    fn parse(item: &str) -> Result<TransactionType, String> {
        return match item {
            "deposit" => Ok(TransactionType::Deposit),
            "withdrawal" => Ok(TransactionType::Withdrawal),
            "dispute" => Ok(TransactionType::Dispute),
            "resolve" => Ok(TransactionType::Resolve),
            "chargeback" => Ok(TransactionType::Chargeback),
            _ => Err("Not a valid transaction type".into()),
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f64>, // u128
    pub disputed: bool,
}

fn parse_amount(amount_str: &str) -> Option<f64> {
    return match amount_str.parse::<f64>() {
        Ok(amount) => Some(amount),
        Err(_) => None,
    };
}

// impl From<StringRecord> for Transaction {
//     fn from(record: StringRecord) -> Self {
//         return Transaction {
//             transaction_type: TransactionType::from(&record[0]),
//             client: record[1].trim().parse::<u16>().unwrap(),
//             tx: record[2].trim().parse::<u32>().unwrap(),
//             amount: match record.len() {
//                 4 => parse_amount(record[3].trim()),
//                 _ => None,
//             },
//             disputed: false,
//         };
//     }
// }

impl Transaction {
    pub fn set_disputed(&mut self) {
        self.disputed = true;
    }
    pub fn remove_disputed(&mut self) {
        self.disputed = false;
    }
    pub fn is_valid(self) -> bool {
        if (matches!(self.transaction_type, TransactionType::Deposit)
            || matches!(self.transaction_type, TransactionType::Withdrawal))
            && self.amount.is_none()
        {
            return false;
        }
        return true;
    }
    pub fn parse(record: StringRecord) -> Result<Transaction, String> {

        if record.len() < 3 {
            return Err("unable to parse row".into());
        }

        let client = match record[1].trim().parse::<u16>() {
            Ok(id) => id,
            Err(_) => return Err("unable to parse client".into()),
        };

        // TODO: What if transaction type is invalid
        let transaction_type = match TransactionType::parse(&record[0]){
            Ok(t) => t,
            Err(_) => return Err("unable to parse transaction type".into())
        };

        let tx = match record[2].trim().parse::<u32>() {
            Ok(id) => id,
            Err(_) => return Err("Unable to parse transaction".into())
        };

        let tx = Transaction {
            transaction_type,
            client,
            tx,
            amount: match record.len() {
                4 => parse_amount(record[3].trim()),
                _ => None,
            },
            disputed: false,
        };
        Ok(tx)
    }
}

#[cfg(test)]
mod tests {
    use super::{Transaction, TransactionType};

    #[test]
    fn hello_world_test() {
        println!("Hello World Test");
    }

    #[test]
    fn should_parse_valid_transaction_type() {
        let tx_type = TransactionType::parse("deposit");
        assert!(matches!(tx_type.ok().unwrap(), TransactionType::Deposit));
    }

    #[test]
    fn should_return_err_if_invalid_transaction_type() {
        let tx_type = TransactionType::parse("does_not_exist");
        assert!(tx_type.is_err());
    }

    #[test]
    fn shound_return_invalid_if_deposit_has_no_amount() {
        let tx = Transaction {
            transaction_type: TransactionType::Deposit,
            client: 100,
            tx: 100,
            amount: None,
            disputed: false,
        };

        assert!(tx.is_valid() == false);
    }

    #[test]
    fn shound_return_invalid_if_withdrawals_has_no_amount() {
        let tx = Transaction {
            transaction_type: TransactionType::Withdrawal,
            client: 100,
            tx: 100,
            amount: None,
            disputed: false,
        };

        assert!(tx.is_valid() == false);
    }

    #[test]
    fn shound_return_valid_if_deposit_has_no_amount() {
        let tx = Transaction {
            transaction_type: TransactionType::Deposit,
            client: 100,
            tx: 100,
            amount: Some(1005.0),
            disputed: false,
        };

        assert!(tx.is_valid());
    }

}
