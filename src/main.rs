use std::io::stdout;
use crossterm::Result;

mod user;
mod command;
mod stock;

fn main() -> Result<()>{
    let mut stdout = stdout();
    command::startup(&mut stdout)
}
