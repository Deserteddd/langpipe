use langpipe::Lexer;

fn main() {
  let lex = Lexer::from("Work in progress!");
  for i in lex {
    println!(
      "{:?} at [{}..{}]: '{}'",
      i.kind(),
      i.position().0,
      i.position().1,
      i.literal(),
    );
  }
}

