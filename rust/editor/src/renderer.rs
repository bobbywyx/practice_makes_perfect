use std::{io, error::Error};

use crossterm::{execute, cursor::MoveTo};

pub struct Renderer{
    
}

impl Renderer {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn draw_tildes(){
        execute!(
            io::stdout(),
            MoveTo(0,0),
        );
        for _ in 1..crossterm::terminal::size().unwrap().1 {
            println!("~\r");
        }
    }
}