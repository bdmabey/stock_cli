use std::fs::{File};
use rand::{thread_rng, Rng};
use std::io::Write;
use crossterm::terminal::{ClearType};
use crossterm::{execute, style, cursor, terminal};
use crate::user::{load_user, save_user, User};
use crate::command::{read_line, game_loop};
use std::time::Duration;
use std::thread;

pub struct Stock {
    pub price: f32,
    pub quantity: i32,
    pub name: String
}

const BUY: &str = r#"Please enter the stock that you would like to buy: "#;

const SELL: &str = r#"Please enter the stock you would like to sell: "#;

pub fn buy<W>(w: &mut W)
where
    W: Write,
{
    let mut user = load_user();
    let mut pos: usize = 0;
    for _i in &user.stocks {pos+=1}

    execute!(
        w,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0,0)
    ).unwrap();

    execute!(
        w,
        style::Print(&user.username),
        cursor::MoveRight(2),
        style::Print("Money: "),
        style::Print(&user.money),
        cursor::MoveTo(0,2),
        style::Print("Name: "),
        cursor::MoveTo(20, 2),
        style::Print("Cost: "),
        cursor::MoveToNextLine(1)
    ).unwrap();

    for i in 0..pos {
        let add: u16 = i as u16;
        execute!(
            w,
            cursor::MoveTo(0, 3+add),
            style::Print(&user.stocks[i].name),
            cursor::MoveTo(20, 3+add),
            style::Print(&user.stocks[i].price)
        ).unwrap();
    }
    execute!(
        w,
        cursor::MoveToNextLine(2),
        style::Print("Please enter which stock you would like to buy:"),
        cursor::MoveToNextLine(1)
    ).unwrap();

    let mut input = read_line();

    for i in 0..pos {
        if input == user.stocks[i].name {
            execute!(
                w,
                cursor::MoveToNextLine(2),
                style::Print("How many would you like to buy? "),
            ).unwrap();
            let buy: i32 = read_line().parse().unwrap();
            let cost = (buy as f32) * user.stocks[i].price;
            if cost > user.money {
                execute!(
                    w,
                    cursor::MoveToNextLine(1),
                    style::Print("You do not have enough money...")
                ).unwrap();
                thread::sleep(Duration::from_secs(2));
                game_loop(w);
            } else {
                user.money -= cost;
                user.stocks[i].quantity += buy;
                save_user(&user);
                game_loop(w);
            }
        }
    }
}

pub fn sell<W>(w: &mut W)
where
    W: Write,
{
    let mut user = load_user();
    let mut pos: usize = 0;
    for _i in &user.stocks {pos+=1}

    execute!(
        w,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0,0)
    ).unwrap();

    execute!(
        w,
        style::Print(&user.username),
        cursor::MoveRight(2),
        style::Print("Money: "),
        style::Print(&user.money),
        cursor::MoveTo(0,2),
        style::Print("Name: "),
        cursor::MoveTo(20, 2),
        style::Print("Cost: "),
        cursor::MoveTo(30, 2),
        style::Print("Quantity Owned:"),
        cursor::MoveToNextLine(1)
    ).unwrap();

    for i in 0..pos {
        let add: u16 = i as u16;
        execute!(
            w,
            cursor::MoveTo(0, 3+add),
            style::Print(&user.stocks[i].name),
            cursor::MoveTo(20, 3+add),
            style::Print(&user.stocks[i].price),
            cursor::MoveTo(30, 3+add),
            style::Print(&user.stocks[i].quantity)
        ).unwrap();
    }
    execute!(
        w,
        cursor::MoveToNextLine(2),
        style::Print("Please enter which stock you would like to sell:"),
        cursor::MoveToNextLine(1)
    ).unwrap();

    let mut input = read_line();

    for i in 0..pos {
        if input == user.stocks[i].name {
            execute!(
                w,
                cursor::MoveToNextLine(2),
                style::Print("How many would you like to sell? "),
            ).unwrap();
            let sell: i32 = read_line().parse().unwrap();
            let cost = (sell as f32) * user.stocks[i].price;
            if sell > user.stocks[i].quantity {
                execute!(
                    w,
                    cursor::MoveToNextLine(1),
                    style::Print("You do not have enough stocks...")
                ).unwrap();
                thread::sleep(Duration::from_secs(2));
                game_loop(w);
            } else {
                user.money += cost;
                user.stocks[i].quantity -= sell;
                save_user(&user);
                game_loop(w);
            }
        }
    }
}

pub fn update_cost() {
    let mut user = load_user();
    let mut pos: usize = 0;

    for _i in &user.stocks {pos+=1}

    for i in 0..pos {
        user.stocks[i].price *= thread_rng().gen_range(0.90..1.15);
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
