use std::io::{Write, stdout};
use crossterm::{execute, queue};
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::cursor::{self};
use crossterm::style;
use crate::command::read_line;
use std::{process, thread};
use std::time::Duration;
use std::fs::File;
use crossterm::event::poll;
use std::process::exit;


const CREATE: &str = r#"We will now create a new user for you.

Please follow the prompts or select a command from below:
Back
Quit"#;

pub struct User {
    pub username: String,
    pub stocks: i32
}

pub fn create<W>(w: &mut W)
where
    W: Write,
{
    let mut user = User{
        username: "".parse().unwrap(),
        stocks: 0
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

        //New read for how many stocks the user wants to work with.
        //Need to look into a way to test the input other than this.
        input = read_line();

        if input == "back" || input == "Back" {
            break
        }

        user.stocks = input.parse().unwrap();
        //saves user info to file.
        save_user(w, &user);
        //need to have this move to main game loop once that is created.
        break
    }
}

pub fn save_user<W>(w: &mut W, user: &User)
where
    W: Write,
{
    execute!(
        w,
        cursor::MoveToNextLine(1),
        style::Print("Saving...")
    ).unwrap();

    let handle = thread::spawn(|| {
        let input = read_line();

        if input == "back" || input == "Back" {
            process::exit(-1)
            //break
        }

        if input == "" {
            process::exit(0)
            //break
        }
    });

    execute!(
        w,
        style::Print("test"),
    ).unwrap();
    thread::sleep(Duration::from_secs(1));

    handle.join().unwrap()
}
