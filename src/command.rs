use std::io::{Write, stdout};
use crossterm::{Result, queue, execute, style};
use crossterm::terminal::{self, ClearType, EnterAlternateScreen};
use crossterm::cursor::{self};
use crossterm::event::{self, Event, KeyEvent, KeyCode, read};
use std::time::Duration;
use std::thread;

const WELCOME: &str = r#"Welcome to my stock simulator.

Please select a command from below:
1)Begin
2)Help
3)Quit"#;

const HELP: &str = r#"This program simulates a few stocks.
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
            cursor::Show,
            cursor::MoveTo(0,1),
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

        execute!(
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
        if input == "back" {
            break
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
                    // cursor::MoveTo(0,7)
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