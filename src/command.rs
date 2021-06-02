use std::io::{Write, stdout};
use crossterm::{Result, queue, execute, style};
use crossterm::terminal::{self, ClearType, EnterAlternateScreen, size};
use crossterm::cursor::{self};
use crossterm::event::{self, Event, KeyEvent, KeyCode, read};
use std::{process, thread};
use crate::user::{create};
use std::sync::mpsc::channel;

const WELCOME: &str = r#"Welcome to my stock simulator.

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

Please tyoe a command from below:
Load
Create
Back
Quit"#;

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
        );

        for line in BEGIN.split('\n') {
            queue!(w, style::Print(line), cursor::MoveToNextLine(1)).unwrap()
        }

        w.flush().unwrap();


        let input = read_line();

        if input == "load" || input == "Load" {
            create(w)
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
            terminal::disable_raw_mode();
            process::exit(0)
        }
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