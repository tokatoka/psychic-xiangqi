use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, WriteColor, BufferWriter};

pub fn write_color(message: &str, color: Option<Color>) -> io::Result<()> {
    let bufwtr = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();
    buffer.set_color(ColorSpec::new().set_fg(color))?;
    write!(&mut buffer, "{}", message)?;
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    bufwtr.print(&buffer)
}

