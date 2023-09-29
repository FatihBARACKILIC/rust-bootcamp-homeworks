struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&mut self) -> f64;
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > self.balance {
            self.balance = 0.0;
        } else {
            self.balance -= amount;
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

    account_of_ali.deposit(654.0);
    account_of_veli.withdraw(987654321.0);

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
