struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&mut self) -> f64;
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        match amount < 0.0 {
            true => Err("A negative amount cannot be withdrawn!".to_string()),
            false => {
                self.balance += amount;
                Ok(())
            }
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount > self.balance {
            Err("You cannot withdraw more than your balance!".to_string())
        } else if amount < 0.0 {
            Err("A negative amount cannot be withdrawn!".to_string())
        } else {
            self.balance -= amount;
            Ok(())
        }
    }

    fn balance(&mut self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account_of_ali: BankAccount = BankAccount {
        account_number: 123,
        holder_name: "Ali".to_string(),
        balance: 10000.0,
    };

    let mut account_of_veli: BankAccount = BankAccount {
        account_number: 321,
        holder_name: "Veli".to_string(),
        balance: 999999999999999.9,
    };

    match account_of_ali.deposit(654.0) {
        Ok(_) => println!("your money has been deposit"),
        Err(err) => panic!("{}", err),
    };

    match account_of_veli.withdraw(987654321.0) {
        Ok(_) => println!("your money has been withdrawn"),
        Err(err) => panic!("{}", err),
    }

    let balance_of_ali = account_of_ali.balance();
    let balance_of_veli = account_of_veli.balance();

    println!(
        "{} {}'s balance: {}",
        account_of_ali.account_number, account_of_ali.holder_name, balance_of_ali
    );
    println!(
        "{} {}'s balance: {}",
        account_of_veli.account_number, account_of_veli.holder_name, balance_of_veli
    );
}
