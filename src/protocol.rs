/*
  Storing a new key:
    set <key> <bytes>\r\n
    <data>\r\n

  E.g.:
    set somekey 10\r\n
    <10 bytes of data>\r\n

  Getting a key:
    get <key>\r\n

    val <key> <bytes>\r\n
    <data>\r\n
*/
use std::io::{BufRead};
use std::fmt;

#[derive(Debug)]
pub enum Command {
  Set,
  Get,
}

#[derive(Debug)]
pub struct ParseError {
  command: String
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "unparseable_command: {}", self.command)
  }
}

pub fn parse<R: BufRead>(reader: &mut R) -> Result<Command, ParseError> {
  let mut raw_command = String::new();
  reader.read_line(&mut raw_command);

  Ok(Command::Get)
}
