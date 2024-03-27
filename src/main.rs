use langpipe::Lexer;

fn main() {
  let lex = Lexer::from(
    "Hey [Name], get 20% off on all orders over $50 this weekend only! Use code SAVE20 at checkout 🔥🔥."
  ); 

  for i in lex {
    println!("{:?}", i)
  }
}



