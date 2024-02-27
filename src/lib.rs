#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lexer<'a> {
    source: &'a str,
    cursor: usize,
    current_kind: TokenKind,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Result<Lexer, String> {
    Ok(Lexer { 
      source: input, 
      cursor: 0, 
      current_kind: get_kind(input
        .chars()
        .nth(0)
        .ok_or("Hello".to_string())?)
    })
  }

  fn char_at_cursor(&self) -> Option<char> {
    self.source.chars().nth(self.cursor)
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.cursor >= self.source.len() {
      return None;
    }
    if self.char_at_cursor() == Some(' ') {
      self.cursor += 1;
      return self.next();
    } else {
      self.current_kind = get_kind(self.char_at_cursor().unwrap());
      /*if self.current_kind == TokenKind::Operator {
        self.cursor += 1;
        return Some(Token::new(TokenKind::Operator, self.source.get(self.cursor-1..self.cursor).unwrap()));
      }*/
      for (index, i) in self.source.chars().skip(self.cursor).enumerate() {
        if get_kind(i) != self.current_kind{
          self.cursor += index;
          if let Some(s) = self.source.get(self.cursor-index..self.cursor) {
            return Some(Token::new(self.current_kind, s));
          }
        }
      }
      self.cursor += 1;
      if let Some(s) = self.source.get(self.cursor-1..) {
        return Some(Token::new(self.current_kind, s));
      }
      None
    }
  }
}

impl<'a> From<&'a str> for Lexer<'a> {
  fn from(value: &'a str) -> Self {
    Lexer::new(value).unwrap()
  }
}

fn get_kind(c: char) -> TokenKind {
  match (
    c.is_ascii_digit(),
    c.is_ascii_punctuation(),
    c.is_alphabetic(),
  ) {
    (true, false, false) => TokenKind::Digit,
    (false, true, false) => TokenKind::Operator,
    (false, false, true) => TokenKind::Word,
    _ => TokenKind::Other,
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token<'a> {
  kind: TokenKind,
  literal: &'a str,
}

impl<'a> Token<'a> {
  fn new(kind: TokenKind, literal: &'a str) -> Self {
    Token {
      kind,
      literal
    }
  }

  pub fn literal(&self) -> &'a str {
    self.literal
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TokenKind {
  Operator,
  Digit,
  Word,
  Other,
}