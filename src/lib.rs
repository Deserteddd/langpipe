use std::fmt::{Display, Debug};

//------------------------------------------------------
//------------------------------------------------------
/// Lexer
//------------------------------------------------------
//------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lexer<'a> {
  source: &'a str,
  byte_cursor: usize,
  char_cursor: usize,
}
impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Result<Lexer, String> {
    Ok(Lexer { 
      source: input, 
      byte_cursor: 0,
      char_cursor: 0,
    })
  }

  fn char_at_cursor(&self) -> Option<char> {
    self.source.chars().nth(self.char_cursor)
  }
}

//------------------------------------------------------
// Trait impls
//------------------------------------------------------
impl<'a> Iterator for Lexer<'a> {
  type Item = Token<'a>;
  fn next(&mut self) -> Option<Self::Item> {
    if self.byte_cursor >= self.source.len() {
      return None;
    }

    let token_kind = match self.char_at_cursor() {
      Some(c) => get_kind(c),
      None => return None,
    };

    let literal = self.source
      .chars()
      .skip(self.char_cursor)
      .take_while(|c| get_kind(*c) == token_kind)
      .collect::<String>();

    let raw_count = literal.len();
    let char_count = literal.chars().count();

    self.byte_cursor += raw_count;
    self.char_cursor += char_count;
      
    if let Some(str) = self.source.get(self.byte_cursor-raw_count..self.byte_cursor){
      return Some(Token::new(token_kind, str, (self.char_cursor-char_count, self.char_cursor-1)));
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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
}
//------------------------------------------------------
// Trait impls
//------------------------------------------------------
impl<'a> Display for Token<'a>{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self.literal)
      
  }
}

impl<'a> Debug for Token<'a>{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(
        f, 
        "{:?} at [{}..{}]: '{}'\n",
        self.kind(),
        self.position().0,
        self.position().1,
        self.literal(),
      )
      
  }
}

//------------------------------------------------------
// Helper functions
//------------------------------------------------------
pub fn get_kind(c: char) -> TokenKind {
  match c.len_utf8() {
    1 | 2 => {
      if c.is_whitespace() { TokenKind::Whitespace }
      else if c.is_alphabetic() { TokenKind::Word }
      else if c.is_ascii_digit() { TokenKind::Number }
      else { TokenKind::Other }
    },

    4 => TokenKind::Emoji,
    _ => TokenKind::Other
  }
  // println!("{:} => {:?}", c, a);
}

//------------------------------------------------------
// TokenKind
//------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
  Punctuation,
  Number,
  Word,
  Whitespace,
  Emoji,
  Other,
}