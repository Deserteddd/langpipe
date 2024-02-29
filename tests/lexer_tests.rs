#[cfg(test)]
mod tests {
  use langpipe::Lexer;

use crate::generate_random_strings;

  #[test]
  fn random_strings() {

    let test_strings = generate_random_strings(500, 250);

    for i in test_strings {
      // Correct output
      let wp_removed: String = i.chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect();

      // Actual output
      let lexed: String = Lexer::from(i.as_str())
        .into_iter()
        .map(|token| token.literal())
        .collect();

      if wp_removed != lexed {
        panic!("Lex error:\nLexed --->{}<--\nActual -->{}<--", wp_removed, lexed);
      } else {
        println!("Ok: {lexed}")
      }
    }
  }

  #[test]
  fn positions() { 
    todo!("Test if position of each token matches actual position in source")
  }
}

use rand::{distributions::Uniform, Rng};
fn generate_random_strings(n: u32 , l: usize) -> Vec<String> {
    let mut test_strings: Vec<String> = vec![];
    // Generate random strings
    (0..n).for_each(|_| test_strings.push(
      rand::thread_rng()
        .sample_iter(&Uniform::from(0..=126))
        .map(|a| if a < 32 {32} else {a})
        .take(l)
        .map(char::from)
        .collect()
    ));
    test_strings
}