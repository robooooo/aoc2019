use crate::utils;
use crossterm::{
    cursor, queue,
    style::{self, Color},
    terminal,
};
use std::{io::prelude::*, ops::Drop};

pub struct PixBuf<W: Write> {
    output: W,
}

impl<W: Write> PixBuf<W> {
    pub fn new<W2: Write>(mut output: W2) -> utils::Result<PixBuf<W2>> {
        terminal::enable_raw_mode()?;
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

    pub fn show_at_cursor(&mut self, color: Color) -> utils::Result<()> {
        queue!(
            self.output,
            style::SetBackgroundColor(color),
            style::Print(" "),
            style::ResetColor,
        )?;
        self.output.flush()?;
        Ok(())
    }

    pub fn fill(&mut self, x0: u16, y0: u16, x1: u16, y1: u16, color: Color) -> utils::Result<()> {
        queue!(
            self.output,
            cursor::RestorePosition,
            cursor::MoveRight(x0),
            cursor::MoveDown(y0),
            style::SetBackgroundColor(color),
        )?;

        for _ in y0..=y1 {
            for _ in x0..=x1 {
                queue!(self.output, style::Print(" "))?;
            }
            queue!(self.output, cursor::MoveDown(1), cursor::MoveLeft(x1 - x0 + 1))?;
        }
        queue!(self.output, style::ResetColor)?;
        self.output.flush()?;
        Ok(())
    }
}

impl<W: Write> Drop for PixBuf<W> {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let _ = queue!(
            self.output,
            cursor::Show,
        );
        let _ = self.output.flush();
    }
}