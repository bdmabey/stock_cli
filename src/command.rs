use std::io::{Write, stdout};
use crossterm::{Result, queue, execute, style};
use crossterm::terminal::{self, ClearType, EnterAlternateScreen};
use crossterm::cursor::{self};
use crossterm::event::{self, Event, KeyEvent, KeyCode, read};
use std::time::Duration;
use std::{thread, process};

const WELCOME: &str = r#"Welcome to my stock simulator.

Please select a command from below (You may enter the number or type out the command):
1)Begin
2)Help
3)Quit"#;

const HELP: &str = r#"This program simulates a few stocks.
From the Welcome screen select Begin to get started.
You will then be asked if you have a previous user and if you would like to use them or not.
If you do want to use your previous user it will be loaded at this point.
If you don't have one/want to create a new one, you will do so at this point.
Commands can either be selected with the number next to it or by typing out the command.

At this point you will be able to buy and sell stocks with the program randomly updating the cost
every 5 seconds.


Please select a command from below:
1)Back
2)Quit"#;

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

        if input == "quit" || input == "3" || input == "Quit"{
            break
        }
        if input == "help" || input == "2" || input == "Help"{
            help(w)
        }
    }

    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()
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
    line
}