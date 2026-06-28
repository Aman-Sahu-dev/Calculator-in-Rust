use Caret::token::Token;

pub struct Lexer{
    input:vec<char>,
    pos:usize,
}
 impl Lexer {
    pub fn new (input: &str) -> Lexer {
        Lexer {
            input : input.chars().collect(),
            pos:0,
        }
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
         if self.pos >= self.input.len(){
            return Token::EOF;
         }
         let ch =  self.input[self.pos];

         match ch {
            '0'..'9' | '.' => self.read_number(),
            'a'..'z' | 'A'..'Z' => self.read_ident(),
            '+' => {self.pos +=1; Token::Plus}
            '-' => {self.pos +=1; Token::Minus}
            '*' => {self.pos +=1; Token::Star}
            '/' => {self.pos +=1; Token::Slash}
            '(' => {self.pos +=1; Token::LParen}
            ')' => {self.pos +=1; Token::RParen}
            ',' => {self.pos +=1; Token::Comma}
            _=> panic!("unexpected character: {}",ch),
         }
    }
    fn read_number(&mut self)-> Token{
        let start = self.pos;
        while self.pos < self.input.len() && (self.input[self.pos].is_ascii_digit() || self.input[self.pos] == '.') {
            self.pos +=1;
        }
    }
 }