use std::{error::Error, io::Write};

pub trait Logger {
    fn log_errln<'a>(&mut self, msg: &'a str) -> Result<(), Box<dyn Error>>;
    fn log_stdln<'a>(&mut self, msg: &'a str) -> Result<(), Box<dyn Error>>;
    fn log_err<'a>(&mut self, msg: &'a str) -> Result<(), Box<dyn Error>>;
    fn log_std<'a>(&mut self, msg: &'a str) -> Result<(), Box<dyn Error>>;
}

pub struct LogWrapper<ERR_W, STD_W>
where
    ERR_W: Write,
    STD_W: Write,
{
    err_writer: ERR_W,
    std_writer: STD_W,
}

impl<ERR_W, STD_W> LogWrapper<ERR_W, STD_W>
where
    ERR_W: Write,
    STD_W: Write,
{
    pub fn new(err_writer: ERR_W, std_writer: STD_W) -> Self {
        LogWrapper {
            err_writer,
            std_writer,
        }
    }
}

impl<ERR_W, STD_W> Logger for LogWrapper<ERR_W, STD_W>
where
    ERR_W: Write,
    STD_W: Write,
{
    fn log_errln<'a>(&mut self, msg: &'a str) -> Result<(), Box<dyn Error>> {
        writeln!(self.err_writer, "{}", &msg)?;
        Ok(())
    }
    fn log_stdln<'a>(&mut self, msg: &'a str) -> Result<(), Box<dyn Error>> {
        writeln!(self.std_writer, "{}", &msg)?;
        Ok(())
    }
    fn log_err<'a>(&mut self, msg: &'a str) -> Result<(), Box<dyn Error>> {
        write!(self.err_writer, "{}", &msg)?;
        Ok(())
    }
    fn log_std<'a>(&mut self, msg: &'a str) -> Result<(), Box<dyn Error>> {
        write!(self.std_writer, "{}", &msg)?;
        Ok(())
    }
}
