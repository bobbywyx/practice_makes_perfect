use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType, SetTitle},
};
use std::{
    error::Error,
    io::{self},
    time::Duration,
};

pub struct Editor {
    should_quit: bool,
}
impl Editor {
    pub fn new() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        init_terminal()?;

        loop {
            if crossterm::event::poll(Duration::from_millis(100))? {
                let event = read()?;
                if let Event::Key(key_event) = event {
                    println!("{:?}\r", event);
                    if let KeyCode::Char(c) = key_event.code {
                        println!("{}\r", c);
                        if c == 'q' {
                            self.should_quit = true;
                            break;
                        }
                    }
                }
            } else {
                println!("100ms no input\r");
            }
        }
        Ok(())
    }
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
