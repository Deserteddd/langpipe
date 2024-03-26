//------------------------------------------------------
//------------------------------------------------------
/// Lexer
//------------------------------------------------------
//------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lexer<'a> {
  source: &'a str,
  cursor: usize,
  offset: usize,
  current_kind: TokenKind,
}
impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Result<Lexer, String> {
    Ok(Lexer { 
      source: input, 
      cursor: 0, 
      offset: 0,
      current_kind: get_kind(input.chars().nth(0).unwrap_or(' '))
    })
  }

  fn char_at_cursor(&self) -> Option<char> {
    self.source.chars().nth(self.cursor)
  }
}

//------------------------------------------------------
// Helper functions
//------------------------------------------------------
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
    if self.cursor+self.offset >= self.source.len() {
      return None;
    }
    if self.char_at_cursor() == Some(' ') {
      self.cursor += 1;
      return self.next();
    }
    self.current_kind = get_kind(self.char_at_cursor().unwrap());
    for (index, (char_index, i)) in self.source.char_indices().skip(self.cursor).enumerate() {
      let mut added_offset = 0;
      if index + self.offset < char_index - self.cursor {
        added_offset = 1;
        self.offset += 1;
      }
      if get_kind(i) != self.current_kind {
        let old_cursor = self.cursor;
        self.cursor += index; //+ (char_index - prev_chr_idx) - 1;
        if let Some(literal) = self.source.get(old_cursor+self.offset-added_offset..self.cursor+self.offset) {

          return Some(Token::new(
            self.current_kind, 
            literal, 
            (old_cursor+self.offset-added_offset, self.cursor+self.offset-1)
          ));
        }
      }
    }
    if let Some(s) = self.source.get(self.cursor+self.offset..) {
      let cursor = self.cursor;
      self.cursor += s.len();
      return Some(Token::new(self.current_kind, s, (cursor+self.offset, self.cursor+self.offset-1)));
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
  pub fn new(kind: TokenKind, literal: &'a str, pos: (usize, usize)) -> Self {
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

  pub fn get_literal_from_source(&self, source: &'a str) -> &'a str {
    source.get(self.pos.0..=self.pos.1).unwrap()
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
  Newline,
  Other,
}