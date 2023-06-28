use std::any::Any;
use std::{error::Error, io::Write};

pub trait Logger: Any {
    fn log_errln<'a>(&mut self, msg: &'a str) -> Result<(), Box<dyn Error>>;
    fn log_stdln<'a>(&mut self, msg: &'a str) -> Result<(), Box<dyn Error>>;
    fn log_err<'a>(&mut self, msg: &'a str) -> Result<(), Box<dyn Error>>;
    fn log_std<'a>(&mut self, msg: &'a str) -> Result<(), Box<dyn Error>>;
    fn as_any(&self) -> &dyn Any;
}

#[derive(PartialEq, Debug)]
pub struct LogWrapper<ERRW, STDW>
where
    ERRW: Write + 'static,
    STDW: Write + 'static,
{
    pub err_writer: ERRW,
    pub std_writer: STDW,
}

impl<ERRW, STDW> LogWrapper<ERRW, STDW>
where
    ERRW: Write,
    STDW: Write,
{
    pub fn new(err_writer: ERRW, std_writer: STDW) -> Self {
        LogWrapper {
            err_writer,
            std_writer,
        }
    }
}

impl<ERRW, STDW> Logger for LogWrapper<ERRW, STDW>
where
    ERRW: Write,
    STDW: Write,
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

    fn as_any(&self) -> &dyn Any {
        self
    }
}
