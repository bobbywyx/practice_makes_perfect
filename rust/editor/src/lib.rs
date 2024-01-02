use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{Clear, ClearType, SetTitle},
};
use renderer::Renderer;
use std::{
    error::Error,
    io::{self},
    time::Duration,
};

mod renderer;

pub struct Editor {
    should_quit: bool,
    renderer: Renderer,
}
impl Editor {
    pub fn new() -> Self {
        Self { should_quit: false , renderer: Renderer::new()}
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        init_terminal()?;
        self.renderer.draw_tildes().display_welcome_msg();

        loop {
            if crossterm::event::poll(Duration::from_millis(10))? {
                self.handle_event(read()?)?;
            }

            self.handle()?;

            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, e: Event) -> Result<(), Box<dyn Error>> {
        if let Event::Key(key_event) = e {
            if key_event.modifiers.is_empty() {
                // println!("{:?}",e);
                // no modifiers
                if let KeyCode::Char(c) = key_event.code {
                    match key_event.kind {
                        KeyEventKind::Press => {
                            if c == 'q' {
                                self.should_quit = true;
                            }
                            match c {
                                'q' => self.should_quit = true,
                                'r' => refresh_terminal()?,
                                'd' => self.renderer.draw_tildes().display_welcome_msg(),
                                _ => (),
                            }
                        }
                        KeyEventKind::Release => (),
                        KeyEventKind::Repeat => (),
                    }
                } else if let KeyCode::Enter = key_event.code {
                    println!("\r");
                }
            }
            if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                // control is pressed
            }
        }
        Ok(())
    }

    fn handle(&mut self) -> Result<(), Box<dyn Error>> {
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

fn refresh_terminal() -> Result<(), Box<dyn Error>> {
    execute!(
        io::stdout(),
        Clear(ClearType::All),
        Clear(ClearType::Purge),
        MoveTo(0, 0)
    )?;
    Ok(())
}
