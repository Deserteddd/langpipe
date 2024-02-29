#[cfg(test)]
//---------------------------------
//---------------------------------
// LEXER TESTS
//---------------------------------
//---------------------------------
mod tests {
  use langpipe::Lexer;
// -----------------------------------------------------------------------------------
  #[test]
  fn random_strings() {
    let test_strings = [
      generate_random_strings(500, 250),
      get_sample_strings()
    ].concat();

    test_strings.iter().for_each(|string| {
      assert_eq!(
        // Correct output
        string.chars()
          .filter(|c| !c.is_ascii_whitespace())
          .collect::<String>()
        ,
        // Actual output
        Lexer::from(string.as_str())
          .into_iter()
          .map(|token| token.literal())
          .collect::<String>()
      )
    })
  }
// -----------------------------------------------------------------------------------
  #[test]
  fn positions() { 
    let test_strings = [
      generate_random_strings(500, 250),
      get_sample_strings()
    ].concat();

    test_strings.iter().enumerate().for_each(|(nth, string)| 
      Lexer::from(string.as_str()).for_each(|token| {
        let (start, end) = token.position();
        assert_eq!(
          token.literal().chars().nth(0).unwrap(),
          test_strings[nth].chars().nth(start).unwrap()
        );
        assert_eq!(
          token.literal().chars().last().unwrap(),
          test_strings[nth].chars().nth(end).unwrap()
        );
        assert_eq!(
          token.literal(),
          test_strings[nth].get(start..=end).unwrap()
        )
      })
    )
  }

//---------------------------------
// Test helpers
//---------------------------------
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

  fn get_sample_strings() -> Vec<String> {
    std::fs::read_to_string("tests/samples.txt")
      .unwrap()
      .lines()
      .map(|content| String::from(content))
      .collect::<Vec<String>>()
  }
}

