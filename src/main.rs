use langpipe::Lexer;

fn main() {
  let sampletext = std::fs::read_to_string("tests/samples.txt").unwrap();
  let lex = Lexer::from(sampletext.as_str());
  for i in lex {
    println!(
      "{:?} at [{}..{}]: '{}'\n",
      i.kind(),
      i.position().0,
      i.position().1,
      i.literal(),
    );
  }
}

