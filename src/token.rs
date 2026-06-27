#[derive(Debug,Clone,PartialEq)]
enum Token {
   Number(f64),
   Ident(String),

   //operators
   Plus,
   Minus,
   Star,
   Slash,
   Caret,
   Equals,

   //diameteres
   LParen,
   RParen,
   Comma,

   //keywords
   Let,
   Fn,
   //End
   Eof,
}