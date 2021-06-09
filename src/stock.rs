use std::fs::{File, read};
use rand::{thread_rng, Rng};
use std::io::Write;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, style, cursor, terminal};
use crate::user::{load_user, save_user};
use crate::command::read_line;

pub struct Stock {
    pub price: f32,
    pub quantity: i32,
    pub name: String
}

const BUY: &str = r#"Please enter the stock number that you would like to buy: "#;

const SELL: &str = r#""#;

pub fn buy<W>(w: &mut W)
where
    W: Write,
{
    let mut user = load_user();

    execute!(
        w,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0,0)
    ).unwrap();

    for line in BUY.split('\n') {
        execute!(w, style::Print(line), cursor::MoveToNextLine(1)).unwrap()
    }

    execute!(
        w,
        style::Print(&user.username),
        cursor::MoveRight(2),
        style::Print("Money: "),
        style::Print(&user.money),
        cursor::MoveTo(0,1),
    ).unwrap();

    let input = read_line();

}

pub fn sell<W>(w: &mut W)
where
    W: Write,
{

}

pub fn update_cost() {
    let mut user = load_user();
    let mut pos: usize = 0;

    for _i in &user.stocks {pos+=1}

    for i in 0..pos {
        user.stocks[i].price *= thread_rng().gen_range(0.95..1.15);
    }
    save_user(&user);
}

pub fn create_stocks(to_create: i32) -> Vec<Stock> {
    let mut file = File::create("src/stock.txt").unwrap();
    let mut stocks: Vec<Stock> = vec![];

    for _i in 0..to_create{
        let mut stock = Stock{
            price: 0.0,
            quantity: 0,
            name: "".parse().unwrap()
        };

        stock.price = thread_rng().gen_range(50.00..500.00);

        let mut test: Vec<u8> = Vec::with_capacity(3);

        for _i in 0..=2 {
            test.push(thread_rng().gen_range(65..=90))
        }

        stock.name = String::from_utf8(test).unwrap();

        write!(file, "{}, {}, {}, ",
               stock.name, stock.quantity, stock.price).unwrap();

        stocks.push(stock);
    }

    stocks
}
