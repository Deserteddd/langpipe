#[cfg(test)]
mod tests {
  use langpipe::Lexer;
  use rand::{distributions::Uniform, Rng};

  #[test]
  fn random_strings() {
    let mut test_strings: Vec<String> = vec![];

    (0..100).for_each(|_| test_strings.push(
      rand::thread_rng()
        .sample_iter(&Uniform::from(0..=126))
        .map(|a| if a < 32 {32} else {a})
        .take(10)
        .map(char::from)
        .collect()
    ));

    for i in test_strings {
      let wp_removed: String = i.chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect();

      let lexed: String = Lexer::from(i.as_str())
        .into_iter()
        .map(|token| token.literal())
        .collect();

      if wp_removed != lexed {
        panic!("Lex error:\nLexed --->{}<--\nActual -->{}<--", wp_removed, lexed)
      }
    }
  }
}