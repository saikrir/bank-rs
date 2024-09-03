#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, accont: Account) {
        self.accounts.push(accont);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.account_summary())
            .collect::<Vec<String>>()
    }
}

#[derive(Debug)]
struct Account {
    id: u32,
    holder: String,
    balance: i32,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }

    fn account_summary(&self) -> String {
        let fmted_summary = format!(
            "Account owner: {} has a balance of ${}  ",
            self.holder, self.balance
        );
        return fmted_summary;
    }
}

fn main() {
    let mut bank = Bank::new();
    let mut acc1 = Account::new(1, String::from("Sai Katterishetty"));
    acc1.deposit(100);

    let mut acc2 = Account::new(2, String::from("Jaya Seyyadri"));
    acc2.deposit(500);
    acc2.withdraw(300);

    bank.add_account(acc1);
    bank.add_account(acc2);

    bank.summary()
        .iter()
        .for_each(|summary| println!("Acc {}", summary));
}
