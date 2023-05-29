use std::{error::Error, io::Write, borrow::BorrowMut};

pub trait Logger {
    fn log<'a>(&mut self, msg: &'a str) -> Result<&'a str, Box<dyn Error>>;
}

pub struct ErrorLogger
{
    write: Box<dyn Write>,
}

impl ErrorLogger
{
    pub fn new(writer: Box<dyn Write>) -> Self {
        ErrorLogger { write: writer }
    }
}

impl Logger for ErrorLogger {
    fn log<'a>(&mut self, msg: &'a str) -> Result<&'a str, Box<dyn Error>> {
        writeln!(self.borrow_mut().write, "{}", &msg)?;
        Ok(&msg)
    }
}
