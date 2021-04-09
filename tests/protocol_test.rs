#[cfg(test)]

mod tests {
  use memcrapped::protocol::{parse, ParseError, Command};
  use std::io::BufReader;

  #[test]
  fn test_parse_get() -> Result<(), ParseError> {
    let mut reader = BufReader::new("something".as_bytes());

    matches!(parse(&mut reader)?, Command::Get);
    Ok(())
  }
}
