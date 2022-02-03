use std::collections::HashMap;

type MemberId = u32;

#[derive(Debug)]
struct Ledger {
    transactions: Vec<Transaction>,
    members: HashMap<MemberId, Member>,
    balances: HashMap<MemberId, f32>,
    ids: u32,
}

impl Ledger {
    fn new() -> Self {
        Ledger {
            transactions: vec![],
            members: HashMap::new(),
            balances: HashMap::new(),
            ids: 0,
        }
    }

    fn add_member(&mut self, name: &str) {
        self.ids += 1;
        self.members
            .insert(self.ids, Member::new(name.to_string(), self.ids));
    }
}

#[derive(Debug)]
struct Transaction {
    title: String,
    amount: i64,
    from: MemberId,
    to: MemberId,
}

impl Transaction {
    fn new(title: String, amount: i64, from: u32, to: u32) -> Self {
        Transaction{
            title,
            amount,
            from,
            to,
        }
    }
}

#[derive(Debug)]
struct Member {
    name: String,
    id: MemberId,
    transactions: Vec<Transaction>,
}

impl Member {
    fn new(name: String, id: u32) -> Self {
        Member {
            name,
            transactions: vec![],
            id,
        }
    }
}

fn main() {
    let mut ledger = Ledger::new();
    ledger.add_member("Megu");
    ledger.add_member("Eli");

    let test = Transaction::new(String::from("Drive-Thru"), 1638, 0, 1);

    ledger.transactions.push(test);

    println!("There are {} members", ledger.ids);
    println!("{:?}", ledger);
}
