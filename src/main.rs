use std::collections::LinkedList;

#[derive(Debug)]
struct Expense {
    title: String,
    amount: f32,
    paid_by: &Member,
    owe: LinkedList<Member>,
}

impl Expense {
    fn new(title:String, amount: f32, paid_by: &Member, owe:LinkedList<Member> ) -> Self {
        Expense{
            title,
            amount,
            paid_by,
            owe,
        }
    }
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
    let eli = Member::new(String::from("Eli"));

    let food = Expense::new(String::from("Drive-Thru"), 16.38, &eli, LinkedList::new());

    println!("{} has {} expenses.", megu.name, megu.expenses.len());
    println!("{} has {} expenses.", eli.name, eli.expenses.len());
}