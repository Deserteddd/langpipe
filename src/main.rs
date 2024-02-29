use langpipe::Lexer;

fn main() -> Result<(), String> {
  for i in Lexer::from("Work in progress!") {
    println!(
      "{:?} at [{}..{}]: '{}'",
      i.kind(),
      i.position().0,
      i.position().1,
      i.literal(),
    )
  }
  Ok(())
}