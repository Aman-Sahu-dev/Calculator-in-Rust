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
        let num: f64 = self.input[start..self.pos]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap();
        Token::Number(num)
    }
    fn read_ident(&mut self) -> Token{
        let start = self.pos;
        while  self.pos < self.input.len() && (self.input[self.pos].is_alphanumeric || self.input[self.pos] == '_'){
            self.pos +=1;
        }
        let word:String = self.input[start..self.pos].iter().collect();
        match word.as_str(){
            "let" => Token::Let,
            "fn" => Token::Fn,
            _=> Token::Ident(word),
        }
    }
    fn skip_whitespace(&mut self){
        while self.pos < self.input.len() && self.input[slef.pos].is_whitespace(){
            self.pos +=1;
        }
    }
 }