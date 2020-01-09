use crate::utils;
use crossterm::{
    cursor, queue,
    style::{self, Color},
};
use std::io::prelude::*;

pub struct PixBuf<W: Write> {
    output: W,
}

impl<W: Write> PixBuf<W> {
    pub fn new<W2: Write>(mut output: W2) -> utils::Result<PixBuf<W2>> {
        queue!(output, cursor::SavePosition, cursor::Hide)?;
        // println!();
        output.flush()?;

        Ok(PixBuf { output })
    }

    pub fn show(&mut self, x: u16, y: u16, color: Color) -> utils::Result<()> {
        queue!(
            self.output,
            cursor::RestorePosition,
            cursor::MoveRight(x),
            cursor::MoveDown(y),
            style::SetBackgroundColor(color),
            style::Print(" "),
            style::ResetColor,
        )?;
        self.output.flush()?;
        Ok(())
    }
}
