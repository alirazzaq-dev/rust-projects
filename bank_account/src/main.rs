struct BankAccount {
    balance: f64,
}
impl BankAccount {
    // Associated function to create a new BankAccount
    fn new() -> Self {
        BankAccount { balance: 0.0 }
    }

    // Methods to deposit, withdraw, and check balance
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    // Withdraw method with a check for sufficient funds
    fn withdraw(&mut self, amount: f64) {
        if amount <= self.balance {
            self.balance -= amount;
        } else {
            println!("Insufficient funds");
        }
    }

    // Method to check the balance
    fn get_balance(&self) -> f64 {
        self.balance
    }

    // Method to transfer funds between accounts
    fn transfer(&mut self, amount: f64, other: &mut BankAccount) {
        if amount <= self.balance {
            self.withdraw(amount);
            other.deposit(amount);
        } else {
            println!("Insufficient funds for transfer");
        }
    }

    // Method to display account information
    fn display_account_info(account: &BankAccount) {
        println!("Current balance: ${:.2}", account.get_balance());
    }
}

fn main() {
    // Create two bank accounts
    let mut account1 = BankAccount::new();
    let mut account2 = BankAccount::new();

    // Deposit money into account1
    account1.deposit(1000.0);
    BankAccount::display_account_info(&account1);

    // Withdraw money from account1
    account1.withdraw(200.0);
    BankAccount::display_account_info(&account1);

    // Transfer money from account1 to account2
    account1.transfer(300.0, &mut account2);
    BankAccount::display_account_info(&account1);
    BankAccount::display_account_info(&account2);
}
