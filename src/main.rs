use std::ops::AddAssign;

struct Account<'a, C> {
    balance: C,
    transactions: std::slice::Iter<'a, C>,
}

impl<C> Iterator for Account<'_, C>
where
    C: AddAssign + Copy,
{
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        match self.transactions.next() {
            Some(amount) => {
                self.balance.add_assign(*amount);
                Some(self.balance)
            }
            None => None,
        }
    }
}

trait Accountant<C> {
    fn account(&self, balance: C) -> Account<C>;
}

impl<C> Accountant<C> for [C] {
    fn account(&self, balance: C) -> Account<C> {
        Account {
            balance,
            transactions: self.iter(),
        }
    }
}

fn main() {
    let transactions = [3.50, 4.00, 5.99];
    let balances: Vec<f64> = transactions.account(0.0).collect();
    println!("{:?}", balances);
}
