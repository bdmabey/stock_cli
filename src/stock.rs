use crate::user::User;
use std::fs::File;
use rand::{thread_rng, Rng};
use std::io::Write;

pub struct Stock {
    pub price: f32,
    pub quantity: i32,
    pub name: String
}

pub fn buy(user: &User) {

}

fn sell(user: &User) {

}

pub fn create_stocks(to_create: i32) -> Vec<Stock> {
    let mut file = File::create("src/stock.txt").unwrap();
    let mut stocks: Vec<Stock> = vec![];

    for i in 0..to_create{
        let mut stock = Stock{
            price: 0.0,
            quantity: 0,
            name: "".parse().unwrap()
        };

        stock.price = thread_rng().gen_range(50.00..500.00);

        let mut test: Vec<u8> = Vec::with_capacity(3);

        for i in 0..=2 {
            test.push(thread_rng().gen_range(65..=90))
        }

        stock.name = String::from_utf8(test).unwrap();

        write!(file, "Name: {}, Quantity: {}, Price: {}, ",
               stock.name, stock.quantity, stock.price);

        stocks.push(stock);
    }

    stocks
}
