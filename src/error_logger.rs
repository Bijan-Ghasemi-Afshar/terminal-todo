use std::{error::Error, io::Write};

pub trait Log {
    fn log<'a>(&mut self, msg: &'a str) -> Result<&'a str, Box<dyn Error>>;
}

pub struct ErrorLogger<W>
where
    W: Write,
{
    pub write: W,
}

impl<W> ErrorLogger<W>
where
    W: Write,
{
    pub fn new(writer: W) -> ErrorLogger<W> {
        ErrorLogger { write: writer }
    }
}

// impl<'a, W: Write> Log for ErrorLogger<'a, W> {
//     fn log<'a>(&mut self, msg: &'a str) -> Result<&'a str, Box<dyn Error>> {
//         write!(self.write, "{}", &msg)?;
//         Ok(&msg)
//     }
// }
