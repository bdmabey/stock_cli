use std::io::{Write, Read};
use crossterm::{execute, queue};
use crossterm::terminal::{self, ClearType};
use crossterm::cursor::{self};
use crossterm::style;
use crate::command::read_line;
use std::fs::File;
use crate::{command, stock};
use crate::stock::Stock;

const CREATE: &str = r#"We will now create a new user for you.

Please follow the prompts or select a command from below:
Back
Quit"#;

pub struct User {
    pub username: String,
    pub stocks_create: i32,
    pub money: f32,
    pub stocks: Vec<Stock>,
    pub runnable: bool
}

pub fn create<W>(w: &mut W)
where
    W: Write,
{
    let mut user = User{
        username: "".parse().unwrap(),
        stocks_create: 0,
        money: 1000.00,
        stocks: vec![],
        runnable: true
    };

    loop {
        queue!(
            w,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0,0)
        ).unwrap();

        for line in CREATE.split('\n') {
            queue!(w, style::Print(line), cursor::MoveToNextLine(1)).unwrap()
        }

        w.flush().unwrap();

        execute!(
            w,
            cursor::MoveToNextLine(1),
            style::Print("Please enter your username:"),
            cursor::MoveToNextLine(1)
        ).unwrap();

        //Initial read for username
        let mut input = read_line();

        if input == "back" || input == "Back" {
            break
        }

        user.username = input;
        execute!(
            w,
            cursor::MoveTo(0,6),
            style::Print("Please enter how many stocks you would like to work with:"),
            cursor::MoveToNextLine(1),
            terminal::Clear(ClearType::CurrentLine)
        ).unwrap();

        input = read_line();

        if input == "back" || input == "Back" {
            break
        }

        user.stocks_create = input.parse().unwrap();
        user.runnable = true;
        user.stocks = stock::create_stocks(user.stocks_create);
        //saves user info to file.
        save_user(&user);
        //need to have this move to main game loop once that is created.
        command::game_loop(w);
    }
}

pub fn load_user() -> User{
    let mut user = User {
        username: "".to_string(),
        money: 1000.00,
        stocks_create: 0,
        stocks: vec![],
        runnable: true
    };

    let mut user_file = File::open("src/user.txt").unwrap();

    let mut stock_file = File::open("src/stock.txt").unwrap();

    let mut input = String::new();

    let mut info = vec![];

    user_file.read_to_string(&mut input).unwrap();

    for line in input.split('\n') {
        info.push(line);
    }

    user.username = info[0].parse().unwrap();
    user.stocks_create = info[1].parse().unwrap();
    user.money = info[2].parse().unwrap();
    user.runnable = info[3].parse().unwrap();

    input.clear();

    stock_file.read_to_string(&mut input).unwrap();

    if input.is_empty() {
        user.stocks = stock::create_stocks(user.stocks_create);
    } else {
        let x: Vec<&str> = input.split(", ").collect();
        let mut size: usize = 0;
        for _i in &x {
            size += 1
        }
        for i in (0..size-1).step_by(3) {
            let stock = Stock{
                price: x[i+2].parse().unwrap(),
                quantity: x[i+1].parse().unwrap(),
                name: x[i].parse().unwrap()
            };
            user.stocks.push(stock);
        }
    }
    user
}

pub fn save_user(user: &User) {
    let mut user_file = File::create("src/user.txt").unwrap_or_else(|error| {
        panic!("Unable to create file: {}", error);
    });

    let mut stock_file = File::create("src/stock.txt").unwrap();

    let mut pos: usize = 0;
    for _i in &user.stocks {pos += 1}

    for i in 0..pos {
        write!(stock_file, "{}, {}, {}, ",
        user.stocks[i].name, user.stocks[i].quantity, user.stocks[i].price).unwrap();
    }

    write!(user_file, "{}\n{}\n{}\n{}",
           user.username, user.stocks_create, user.money, user.runnable).unwrap();

}
