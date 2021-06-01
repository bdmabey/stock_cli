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
3)Exit"#;

enum Command {

}

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
            cursor::Hide,
            cursor::MoveTo(0,1),
        )?;

        for line in WELCOME.split('\n') {
            queue!(w, style::Print(line), cursor::MoveToNextLine(1))?
        }

        w.flush()?;

        let input = read_line();

        if input == "quit" {
            break
        }
        if input == "" {

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

pub fn read_line() -> String {
    let mut line = String::new();
    let mut stdout = stdout();
    while let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
        match code {
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