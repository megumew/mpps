//Double entry ledger
use std::collections::HashMap;

type MemberId = u32;

#[derive(Debug)]
struct Ledger {
    transactions: Vec<Transaction>,
    members: HashMap<MemberId, Member>,
    balances: HashMap<MemberId, i64>,
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

    fn add_member(&mut self, name: &str) -> MemberId {
        self.ids += 1;
        self.members
            .insert(self.ids, Member::new(name.to_string(), self.ids));

        self.balances
            .insert(self.ids,0);

        self.ids
    }

    //Create new transactions and add them to the ledger data
    fn add_transaction(&mut self, title: String, amount: i64, from: u32, to: u32) -> usize {
        let transaction = Transaction {
            title,
            amount,
            from,
            to,
        };

        self.transactions.push(transaction);
        self.update_balances();
        
        self.transactions.len() - 1
    }

    //Updates the hash of user balances
    fn update_balances(&mut self) -> () {
        for t in &self.transactions{
            self.balances.insert(t.from, -t.amount + if self.balances.contains_key(&t.from) { self.balances[&t.from] } else { 0 });
            self.balances.insert(t.to, t.amount + if self.balances.contains_key(&t.to) { self.balances[&t.to] } else { 0 });
        }
    }

    fn get_member_name(&self, id: &u32) -> String {
        let member = self.members.get_key_value(&id);
        member.unwrap().1.get_name()
    }
}

#[derive(Debug)]
struct Transaction {
    title: String,
    amount: i64,
    from: MemberId,
    to: MemberId,
}

impl Transaction {}

#[derive(Debug)]
struct Member {
    name: String,
    id: MemberId,
    //transactions: Vec<Transaction>,
}

impl Member {
    fn new(name: String, id: u32) -> Self {
        Member {
            name,
            //transactions: vec![],
            id,
        }
    }

    fn get_name(& self) -> String {
        self.name.clone()
    }

}

fn main() {
    let mut ledger = Ledger::new();
    let megu = ledger.add_member("Megu");
    let eli = ledger.add_member("Eli");

    let test = ledger.add_transaction(String::from("Drive-Thru"), 1638, eli, megu);

    println!("There are {} members", ledger.ids);
    println!("{:?}", ledger);
    println!("{:?}", ledger.transactions.get(test));
    println!("{}", test);

    for bal in &ledger.balances{
        println!("{}(id: {}) has a balance of {}.", ledger.get_member_name(&bal.0), bal.0, bal.1);
    }
}
