use std::{io, error::Error};

use crossterm::{execute, cursor::MoveTo, terminal::{Clear, ClearType}};

pub struct Renderer{
    size: (u16,u16),

}

impl Renderer {
    pub fn new() -> Self {
        Self { size : crossterm::terminal::size().unwrap() }
    }

    pub fn draw_tildes(&self) -> &Self{
        execute!(
            io::stdout(),
            MoveTo(0,0),
        ).unwrap();
        for _ in 1..crossterm::terminal::size().unwrap().1 {
            println!("~\r");
        }
        self
    }

    pub fn clear(&self){
        execute!(
            io::stdout(),
            Clear(ClearType::All),
            Clear(ClearType::Purge),
            MoveTo(0, 0)
        ).unwrap();
    }

    pub fn display_welcome_msg(&self){
        execute!(
            io::stdout(),
            MoveTo(self.size.0/2,self.size.1/2)
        ).unwrap();
        print!("hello\r\n");
    }

    pub fn end(&self){

    }

}