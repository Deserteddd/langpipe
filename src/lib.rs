//------------------------------------------------------
//------------------------------------------------------
/// Lexer
//------------------------------------------------------
//------------------------------------------------------
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
      current_kind: get_first_kind(&input)
    })
  }

  fn char_at_cursor(&self) -> Option<char> {
    self.source.chars().nth(self.cursor)
  }
}

//------------------------------------------------------
// Helper functions
//------------------------------------------------------
fn get_first_kind(s: &str) -> TokenKind {
  get_kind(s.chars().nth(0).unwrap_or(' '))
}

fn _subtract_or_zero(n: usize, p: usize) -> usize {
  match p > n {
    true => 0,
    false => n-p
  }
}

//------------------------------------------------------
// Trait impls
//------------------------------------------------------
impl<'a> Iterator for Lexer<'a> {
  type Item = Token<'a>;
  fn next(&mut self) -> Option<Self::Item> {
    if self.cursor == self.source.len() {
      return None;
    }
    if self.char_at_cursor() == Some(' ') {
      self.cursor += 1;
      return self.next();
    }
    self.current_kind = get_kind(self.char_at_cursor().unwrap());

    for (index, i) in self.source.chars().skip(self.cursor).enumerate() {
      if get_kind(i) != self.current_kind {
        let old_cursor = self.cursor;
        self.cursor += index;
        if let Some(s) = self.source.get(old_cursor..self.cursor) {
          return Some(Token::new(self.current_kind, s, (old_cursor, self.cursor-1)));
        }
      }
    }

    if let Some(s) = self.source.get(self.cursor..) {
      let cursor = self.cursor;
      self.cursor += s.len();
      return Some(Token::new(self.current_kind, s, (cursor, self.cursor-1)));
    }
    None
  }
}

impl<'a> From<&'a str> for Lexer<'a> {
  fn from(value: &'a str) -> Self {
    Lexer::new(value).unwrap()
  }
}

//------------------------------------------------------
//------------------------------------------------------
// Token
//------------------------------------------------------
//------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token<'a> {
  kind: TokenKind,
  literal: &'a str,
  pos: (usize, usize),
}

impl<'a> Token<'a> {
  fn new(kind: TokenKind, literal: &'a str, pos: (usize, usize)) -> Self {
    Token {
      kind,
      literal,
      pos
    }
  }

  pub fn kind(&self) -> TokenKind {
    self.kind
  }

  pub fn literal(&self) -> &'a str {
    self.literal
  }

  pub fn position(&self) -> (usize, usize) {
    self.pos
  }
}

//------------------------------------------------------
// Helper functions
//------------------------------------------------------
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

//------------------------------------------------------
// TokenKind
//------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
  Operator,
  Digit,
  Word,
  Other,
}