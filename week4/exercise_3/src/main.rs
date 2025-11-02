use std::io;

struct BankAccount {
    balance: f64,
}

impl BankAccount {
    fn new(balance: f64) -> Self {
        Self { balance }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited {:.2}", amount);
    }

    fn withdraw(&mut self, amount: f64) {
        if amount <= self.balance {
            self.balance -= amount;
            println!("Withdrew {:.2}", amount);
        } else {
            println!("Accoutn declined");
        }
    }

    fn check_balance(&self) {
        println!("Balance: {:.2}", self.balance);
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter initial balance:");
    io::stdin().read_line(&mut input).unwrap();
    let mut account = BankAccount::new(input.trim().parse().unwrap());

    loop {
        println!("\n1.Deposit  2.Withdraw  3.Check Balance  4.Exit");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        if choice == "1" {
            println!("Enter deposit amount:");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            account.deposit(input.trim().parse().unwrap());
        } else if choice == "2" {
            println!("Enter withdrawal amount:");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            account.withdraw(input.trim().parse().unwrap());
        } else if choice == "3" {
            account.check_balance();
        } else if choice == "4" {
            println!("Goodbye!");
            break;
        } else {
            println!("Invalid option.");
        }
    }
}
