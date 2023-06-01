use std::{error::Error, io::Write};

pub trait Logger {
    fn log<'a>(&mut self, msg: &'a str) -> Result<(), Box<dyn Error>>;
}

pub struct ErrorLogger<W>
where
    W: Write,
{
    write: W,
}

impl<W> ErrorLogger<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        ErrorLogger { write: writer }
    }
}

impl<W> Logger for ErrorLogger<W>
where
    W: Write,
{
    fn log<'a>(&mut self, msg: &'a str) -> Result<(), Box<dyn Error>> {
        writeln!(self.write, "{}", &msg)?;
        Ok(())
    }
}
