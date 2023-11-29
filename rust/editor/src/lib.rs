use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType, SetTitle},
};
use std::{
    error::Error,
    io::{self},
};

pub fn run() -> Result<(), Box<dyn Error>> {
    init_terminal()?;

    loop {
        let event = read()?;
        if let Event::Key(key_event) = event {
            // println!("{:?}", event);
            if let KeyCode::Char(c) = key_event.code {
                println!("{}", c);
                if c == 'q' {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn init_terminal() -> Result<(), Box<dyn Error>> {
    execute!(
        io::stdout(),
        MoveTo(0, 0),
        Clear(ClearType::All),
        Clear(ClearType::Purge),
        SetTitle("editoR"),
    )?;
    crossterm::terminal::enable_raw_mode()?;
    Ok(())
}
