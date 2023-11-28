use std::{
    io::{self, Read},
    process,
};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType, SetTitle},
};

fn main() -> io::Result<()> {
    // initialization
    execute!(
        io::stdout(),
        MoveTo(0, 0),
        Clear(ClearType::All),
        Clear(ClearType::Purge),
        SetTitle("editoR"),
    )?;

    crossterm::terminal::enable_raw_mode()?;
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{}", c);
        if c == 'q' {
            process::exit(0);
        }
    }

    Ok(())
}
