use crate::recorder;
use std::io::{self, Write};

pub struct Writer<W> {
    writer: W,
}

impl<W> Writer<W> {
    pub fn new(writer: W) -> Self {
        Writer { writer }
    }
}

impl<W: Write> recorder::EventWriter for Writer<W> {
    fn start(&mut self, header: &recorder::Header, append: bool) -> io::Result<()> {
        if append {
            Ok(())
        } else {
            write!(self.writer, "\x1b[8;{};{}t", header.rows, header.cols)
        }
    }

    fn output(&mut self, _time: u64, data: &[u8]) -> io::Result<()> {
        self.writer.write_all(data)
    }

    fn input(&mut self, _time: u64, _data: &[u8]) -> io::Result<()> {
        Ok(())
    }

    fn resize(&mut self, _time: u64, _size: (u16, u16)) -> io::Result<()> {
        Ok(())
    }

    fn marker(&mut self, _time: u64) -> io::Result<()> {
        Ok(())
    }
}
