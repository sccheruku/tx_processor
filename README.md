### Build it
```
cargo build
```

### Test it
```
cargo test
```

### Run it
```
cargo run -- transactions.csv > accounts.csv
```

### Assumptions: 

1. It would be ideal to use u128 because of a safer methods like u128.checked_add, which returns None when there is an overflow. We can write safer code.

2. I opted to use f64 because it was easier (for the test project), and also because these transactions are CreditCard transactions, overflow would be unlikely because it would require more funds than [all the money in the world](https://www.gobankingrates.com/money/economy/how-much-money-is-in-the-world/). :grin:
Also, f64 was easier to parse. 

3. I've assumed that disputed transactions only apply to deposits type transactions. 

4. After an account is locked, no transactions are accepted for that account.

5. Generally, fail fast approach is preferred. If this was streaming data to a function or a lambda, we could fail on this transaction and log to a database. but we cannot do that for this example, because it is expected to produce an output csv.

6. I commented that tests that we would need in transaction_processor.rs but did not complete them. 


### Test cases

####  Alice (client_id `4000`)
Test that deposits should work
```
type, client, tx, amount
deposit, 4000, 4001, 1.0000
deposit, 4000, 4002, 2.0000
```
```
client, available, held, total, locked
4000, 3.0000, 0.0000, 3.0000, false
```


#### Bob (client_id: `5000`)
Test that withdrawal works
```
type, client, tx, amount
deposit, 5000, 5001, 1.12345
deposit, 5000, 5002, 1.12345
withdrawal, 5000, 5003, 1.12345
```
```
client, available, held, total, locked
5000, 1.12345, 0.0, 1.12345, false
```

### Dan (client_id: `6000`)
Testing rounding by making a couple of deposits and withdrawing funds
```
type, client, tx, amount
deposit, 6000, 6001, 1.12345
deposit, 6000, 6002, 1.67890
withdrawal, 6000, 6003, 1
```
```
client, available, held, total, locked
6000, 1.8024, 0.0, 1.8024, false

```
#### George (client_id: `7000`)
Test that a chargeback locks the account
```
type, client, tx, amount
deposit, 7000, 7001, 150.0
withdrawal, 7000, 7003, 175.0
deposit, 7000, 7004, 75.0
dispute, 7000, 7004
chargeback, 7000, 7004
```
```
client, available, held, total, locked
7000, 150.0000, 0.0000, 150.0000, true
```

#### Jorge (client_id: `8000`)
Test that resolving a dispute holds the funds on the account
```
type, client, tx, amount
deposit, 8000, 8001, 150.0
withdrawal, 8000, 8002, 175.0
dispute, 8000, 8001
```
```
client, available, held, total, locked
8000, 0.0000, 150.0000, 150.0000, false
```

#### Jason (client_id: `10000`)
Test that resolving a dispute holds the funds on the account
```
type, client, tx, amount
deposit, 10000, 10001, 150.0
withdrawal, 10000, 10002, 175.0
dispute, 10000, 10001
resolve, 10000, 10001
```
```
client, available, held, total, locked
10000, 150.0000, 0.0000, 150.0000, false
```

#### Ed (client_id: `9000`)
Testing for withdrawals, disputes and resolving of the dispute
```
type, client, tx, amount
deposit, 9000, 9001, 150.0
deposit, 9000, 9002, 150.0
deposit, 9000, 9003, 150.0
deposit, 9000, 9004, 150.0
withdrawal, 9000, 9005, 175.0
dispute, 9000, 9001
resolve, 9000, 9001
```
```
client, available, held, total, locked
9000, 275.0000, 150.0000, 425.0000, false
```

#### Anna (client_id: `11000`)
Testing for disputing, resolving and disputing the same transaction again
```
type, client, tx, amount
deposit, 11000, 11001, 150.0
deposit, 11000, 11002, 150.0
deposit, 11000, 11003, 150.0
deposit, 11000, 11004, 150.0
withdrawal, 11000, 11005, 175.0
dispute, 11000, 11001
resolve, 11000, 11001
dispute, 11000, 11001
```
```
client, available, held, total, locked
11000, 275.0000, 150.0000, 425.0000, false
```

#### Unknown (client_id: `12000`)
Testing for testing for chargebacks after disputing a transaction
```
type, client, tx, amount
deposit, 12000, 12001, 150.0
deposit, 12000, 12002, 150.0
deposit, 12000, 12003, 150.0
deposit, 12000, 12004, 150.0
withdrawal, 12000, 12005, 175.0
dispute, 12000, 12001
chargeback, 12000, 12001
```
```
client, available, held, total, locked
12000, 275.0000, 0.0000, 275.0000, true
```

### Live Coding:

Summary - 

[![Summary](https://img.youtube.com/vi/Nbcr16ksZIQ/0.jpg)](https://www.youtube.com/watch?v=Nbcr16ksZIQ)

Part 1 - 2hr

[![Part 1](https://img.youtube.com/vi/usyCPOEJPBw/0.jpg)](https://www.youtube.com/watch?v=usyCPOEJPBw)

Part 2 - 1hr

[![Part 2](https://img.youtube.com/vi/3RdQv2xx7uw/0.jpg)](https://www.youtube.com/watch?v=3RdQv2xx7uw)

Part 3 - 20m

[![Part 3](https://img.youtube.com/vi/QbOrlniaZKM/0.jpg)](https://www.youtube.com/watch?v=QbOrlniaZKM)
