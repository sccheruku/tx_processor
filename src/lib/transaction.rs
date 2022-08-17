use csv::StringRecord;

#[derive(Debug, Clone, Copy)]
pub enum TransactionType {
    Deposit, Withdrawal, 
    Dispute, Resolve, Chargeback
}

impl From<&str> for TransactionType {
    fn from(item: &str) -> Self {
        return match item {
            "deposit" => TransactionType::Deposit,
            "withdrawal" => TransactionType::Withdrawal,
            "dispute" => TransactionType::Dispute,
            "resolve" => TransactionType::Resolve,
            "chargeback" => TransactionType::Chargeback,
            _ => panic!("Not a valid transaction type")
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub client: u16, 
    pub tx: u32, 
    pub amount: Option<f64>, // u128
    pub disputed: bool
}

impl From<StringRecord> for Transaction {
    fn from(record: StringRecord) -> Self {
        return Transaction {
            transaction_type: TransactionType::from(&record[0]),
            client: record[1].trim().parse::<u16>().unwrap(),
            tx: record[2].trim().parse::<u32>().unwrap(),
            amount: match record.len() {
                4 => Some(record[3].trim().parse::<f64>().unwrap()),
                _ => None
            },
            disputed: false
        }
    }
}

impl Transaction {
    pub fn set_disputed(&mut self){
        self.disputed = true;
    }
    pub fn remove_disputed(&mut self){
        self.disputed = false;
    }
}

#[cfg(test)]
mod tests {
    use super::TransactionType;

    #[test]
    fn hello_world_test() {
        println!("Hello World Test");
    }

    #[test]
    fn should_parse_valid_transaction_type(){
        let tx_type = TransactionType::from("deposit");
        assert!(matches!(tx_type, TransactionType::Deposit));
    }

    #[test]
    #[should_panic]
    fn should_panic_if_invalid_transaction_type(){
        let _tx_type = TransactionType::from("does_not_exist");
    }
}
