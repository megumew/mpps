use std::collections::LinkedList;

#[derive(Debug)]
struct Expense {
    title: String,
    amount: f32,
    paid_by: Member,
    owe: LinkedList<Member>,
}

#[derive(Debug)]
struct Member {
    name: String,
    expenses: LinkedList<Expense>,
}

impl Member {
    fn new(name: String) -> Self {
        Member{
            name,
            expenses: LinkedList::<Expense>::new()
        }
    }
}

fn main() {
    let megu = Member::new(String::from("Megu"));
    println!("{:?}", megu);
}