use std::io::{Write, stdout};
use crossterm::{Result, queue, execute, style};
use crossterm::terminal::{self, ClearType, EnterAlternateScreen};
use crossterm::cursor::{self};
use crossterm::event::{self, Event, KeyEvent, KeyCode   };
use std::{process, thread};
use crate::user::{create, load_user, save_user};
use std::time::Duration;
use crate::stock;
use std::mem::drop;
use std::sync::mpsc::channel;

const WELCOME: &str = r#"Welcome to my Stock simulator.

Please select a command from below (You may enter the number or type out the command):
Begin
Help
Quit"#;

const HELP: &str = r#"This program simulates a few stocks.
From the Welcome screen select Begin to get started.
You will then be asked if you have a previous user and if you would like to use them or not.
If you do want to use your previous user it will be loaded at this point.
If you don't have one/want to create a new one, you will do so at this point.
You can select a command by typing in the command.
At any point if you want to exit the program you can type quit to exit the program.

At this point you will be able to buy and sell stocks with the program randomly updating the cost
every 5 seconds.


Please select a command from below:
Back
Quit"#;

const BEGIN: &str = r#"Great let's get started.

Please type a command from below:
Load
Create
Back
Quit"#;

const GAME_LOOP: &str = r#"Username:
Stock info here:
"#;

pub fn startup<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    terminal::enable_raw_mode()?;
    execute!(w, EnterAlternateScreen)?;

    loop {
        queue!(
            w,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0,0),
        )?;

        for line in WELCOME.split('\n') {
            queue!(w, style::Print(line), cursor::MoveToNextLine(1))?
        }

        w.flush()?;

        let input = read_line();

        if input == "help" || input == "Help"{
            help(w)
        }
        if input == "begin" || input == "Begin" {
            begin(w)
        }
    }
}

fn begin<W>(w: &mut W)
where
    W: Write,
{
    loop {
        queue!(
            w,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0,0)
        ).unwrap();

        for line in BEGIN.split('\n') {
            queue!(w, style::Print(line), cursor::MoveToNextLine(1)).unwrap()
        }

        w.flush().unwrap();


        let input = read_line();

        if input == "Load" || input == "load" {
            game_loop(w);
        }

        if input == "create" || input == "Create" {
            create(w)
        }

        if input == "back" || input == "Back" {
            break
        }

    }
}

fn help<W>(w: &mut W)
where
    W: Write,
{
    loop {

        queue!(
            w,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0,0)
        ).unwrap();

        for line in HELP.split('\n') {
            queue!(
                w,
                style::Print(line),
                cursor::MoveToNextLine(1),
            ).unwrap();
        }

        w.flush().unwrap();

        let input = read_line();
        if input == "back" || input == "1" || input == "Back"{
            break
        }
        if input == "quit" || input == "2" || input == "Quit"{
            execute!(
                w,
                style::ResetColor,
                terminal::LeaveAlternateScreen,
            ).unwrap();
            terminal::disable_raw_mode().unwrap();
            process::exit(0)
        }
    }
}

pub fn game_loop<W> (w: &mut W)
where
    W: Write,
{
    let mut user = load_user();
    user.runnable = true;
    save_user(&user);

    queue!(
        w,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0,0),
    ).unwrap();

    for line in GAME_LOOP.split('\n'){
        queue!(w, style::Print(line), cursor::MoveToNextLine(1)).unwrap()
    }

    let handle = thread::spawn(move || {
        let mut stdout = stdout();
        loop {
            let user = load_user();
            let mut pos: usize = 0;
            for _i in &user.stocks {
                pos+=1
            }
            execute!(
                stdout,
                cursor::MoveTo(9,0),
                style::Print(&user.username),
                cursor::MoveRight(2),
                style::Print("Money: "),
                style::Print(&user.money),
                cursor::MoveTo(0,1),
                style::Print("Stock Name:      Cost:        Quantity Owned:"),
                cursor::MoveTo(0,2)
            ).unwrap();

            for i in 0..pos {
                let add: u16 = i as u16;
                execute!(
                stdout,
                cursor::MoveTo(0, 2 + add),
                style::Print(&user.stocks[i].name),
                cursor::MoveTo(17, 2+add),
                style::Print(&user.stocks[i].price),
                cursor::MoveTo(30, 2+add),
                style::Print(&user.stocks[i].quantity),
            ).unwrap();
            }

            execute!(
                stdout,
                cursor::MoveToNextLine(2),
                style::Print("Please enter a command from below:"),
                cursor::MoveToNextLine(1),
                style::Print("Buy"),
                cursor::MoveToNextLine(1),
                style::Print("Sell"),
                cursor::MoveToNextLine(1),
                style::Print("Quit"),
                cursor::MoveToNextLine(1),
            ).unwrap();
            if user.runnable == false {
                break
            }
            thread::sleep(Duration::from_secs(5));
            stock::update_cost();
        }
    });

    let input = read_line();

    if input == "Buy" || input == "buy"{
        user.runnable = false;
        save_user(&user);
        handle.join().unwrap();
        stock::buy(w)
    }
    if input == "Sell" || input == "sell"{
        stock::sell(w)
    }
}

pub fn read_line() -> String {
    let mut line = String::new();
    let mut stdout = stdout();
    while let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
        match code {
            KeyCode::Backspace => {
                execute!(
                stdout,
                cursor::MoveLeft(1),
                terminal::Clear(ClearType::FromCursorDown),
            ).unwrap();
                line = "".parse().unwrap();
            }
            KeyCode::Enter => {
                break;
            }
            KeyCode::Char(c) => {
                execute!(
                stdout,
                style::Print(c),
            ).unwrap();
                line.push(c);
            }
            _ => {}
        }
    }

    if line == "quit" || line ==  "Quit" {
        execute!(
            stdout,
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen,
        ).unwrap();
        terminal::disable_raw_mode().unwrap();
        process::exit(0)
    }

    line
}