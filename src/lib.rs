mod utils;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_use] extern crate lazy_static;
use wasm_bindgen::prelude::*;
extern crate web_sys;
use web_sys::{AudioContext, OscillatorType, console};
extern crate regex;
use regex::Regex;

/* 
Let's start with the simplest possible language that we could have fun with.
it could have four elements:
    - pitches, inc. those played in series (melody) and in parallel (harmony)
    - rhythms
    - timbres/instruments
    - dynamics

the main factors i want to consider in the design:
    - how many keypresses are needed to express a simple idea? a complex idea?
        - whitespace should used sparingly
        - basic ideas should require one or two keypresses, pref not simultaneous
        - brackets should be used sparingly
    - ease of iteration/refinement
        - small changes should be as local as possible in their effects
        - 

For simplicity, but not too much simplicity, 
at this stage, pitches could be represented by abcdefg 
    with , flat ' sharp 
    (< and > could be microtonal operators)
    / down \ up octave (default is with A = 440)

difference between chords and melodies?
    for now, a string starting with x could be a chord, m a melody
    e.g. 
        - chord1: #ce,gb,
            - the units inside a chord can be notes or rhythms
        - melody1: ~ab/df'
        - one interesting possibility: labeling notes so they can be changed individually

rhythms?
    possibilities:
        there is some pulse: in phrases, we specify durations relative to this pulse
            - one possibility: the phrase is interpreted as a bar (determined by a time sig)
                - the beats within this bar?
                    - /a,.'/b,/a,
                    - #/a/a/'/
                        - / determines divisions
                        - names refer to melodies, chords, or other rhythms
                        - a, means "a lasts for two units"
                        - a. means "two units of a"
                        - a' means "a followed by rest"
                        - ', means "rest"
                - how to apply to a melody or a chord?
                    - say you have 
                        - m1=~abcbadc
                        - r1= x /x../'x,/''x./'x
                            this unpacks to four even divisions
                                first has three evenly spaced notes
                                second is divided into three, a rest + a note held for 2
                                third has four

                        - r2:m1\r1
                        - also if you do ~abc\/1.
                        - how do you express concatenation? parallel? oh like
                            ~r1 r2
                            #r1 r2
                            #:/1..:/1. means "3 over 2 polyrhythm"
                            ~:/1..:/1. means "Dadada Dada"
                        - what about subsubdivisions?
                            - two ideas:
                                - brackets
                                - make ? rest and ' halve
                                so /1/
                        - compromise:
                            - one slash is one beat
                            - so /11.
                - possibly we have to be able to express subdivisions directly
                - /5..../3.'2./

                        
    - how do you tell the difference between rhythms and melodies/chords?


    additive rhythms: there is no pulse, phrases are built from units with fixed durations


    
*/

/* tokenizer:
    what are the tokens?
    equals, parens, comma, dot, apostrophe, slashes, 
    hash, tilde, angle bracs, plus, minus 
    
    abcdefg
    
    identifier, number
    
    eof 
    
    
    tokens will be structs, with an enum value, location, lexeme,
    

    */
#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    Equals,
    LeftParen,
    RightParen,
    LeftSquare,
    RightSquare,
    LeftCurly,
    RightCurly,
    Comma,
    Dot,
    Apostrophe,
    ForwardSlash,
    BackSlash,
    Hash,
    Tilde,
    LeftAngle,
    RightAngle,
    Plus,
    Minus,
    Backtick,
    Pipe,
    Question,
    Bang,
    Dollar,
    Ampersand,
    DoubleQuote,
    Percent,
    Caret,
    Semicolon,
    Colon,
    Asterisk,
    NewLine,
    Identifier(String),
    Number(f32),
}

fn char_to_token(ch: &char) -> Option<TokenType> {
    match ch {
        '=' => Some(TokenType::Equals), 
        '(' => Some(TokenType::LeftParen), 
        ')' => Some(TokenType::RightParen),
        '[' => Some(TokenType::LeftSquare), 
        ']' => Some(TokenType::RightSquare),
        '{' => Some(TokenType::LeftCurly),
        '}' => Some(TokenType::RightCurly),
        ',' => Some(TokenType::Comma), 
        '.' => Some(TokenType::Dot),
        '#' => Some(TokenType::Hash),
        '~' => Some(TokenType::Tilde),
        '<' => Some(TokenType::LeftAngle),
        '>' => Some(TokenType::RightAngle),
        '+' => Some(TokenType::Plus),
        '-' => Some(TokenType::Minus),
        '\'' => Some(TokenType::Apostrophe),
        '/' => Some(TokenType::ForwardSlash), 
        '\\' => Some(TokenType::BackSlash),
        '`' => Some(TokenType::Backtick),
        '|' => Some(TokenType::Pipe),
        '?' => Some(TokenType::Question),
        '!' => Some(TokenType::Bang),
        '$' => Some(TokenType::Dollar),
        '&' => Some(TokenType::Ampersand),
        '%' => Some(TokenType::Percent),
        '^' => Some(TokenType::Caret),
        '"' => Some(TokenType::DoubleQuote),
        ';' => Some(TokenType::Semicolon),
        ':' => Some(TokenType::Colon),
        '*' => Some(TokenType::Asterisk),
        _ => None
    }
}

#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
}

type TokenList = Vec<Token>;

fn tok_identifier(input_text: &str) -> Result<(Token, usize), String> {
    console::log_1(&format!("parsing this as identifier: {:#?}", input_text).into());
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([a-zA-Z_]+)").unwrap();
    }
    let s = RE.captures(input_text).ok_or("not valid identifier 1")?
              .get(0).ok_or("not valid identifier 2")?
              .as_str();
    Ok((Token {token_type: TokenType::Identifier(String::from(s))}, s.len()))
}

fn tok_number(input_text: &str) -> Result<(Token, usize), String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([0-9]*(?:\.[0-9]*)?)").unwrap();
    }
    let s_string = RE.captures(input_text).ok_or("not valid number 1")?
              .get(0).ok_or("not valid number 2")?
              .as_str();
    let s_parsed = s_string.parse::<f32>().ok().ok_or("not valid number 3")?;
    Ok((Token {token_type: TokenType::Number(s_parsed)}, s_string.len()))
}

fn tokenize(input: &String) -> Result<TokenList, String> {
    let mut input_string = input.char_indices().peekable();
    let mut token_list: TokenList = Vec::new();
    let mut line_num = 1;
    let mut bad_char_index: u8 = 0;
    let mut last_total_chars: u8 = 0;
    let res: Option<()> = loop {
        match input_string.peek().cloned() {
            Some((ix, s)) => {
                match s {
                    ' ' => Some(()),
                    '\n' => {
                        line_num += 1;
                        last_total_chars = (ix as u8) + 1;
                        Some(token_list.push(Token {token_type: TokenType::NewLine}))
                    },
                    '0'..='9' => {
                        let slice = input.get(ix..).ok_or("could not parse")?;
                        let (ident, id_length) = tok_number(slice)?;
                        for _ in 0..(id_length-1) {
                            input_string.next();
                        }
                        Some(token_list.push(ident))
                    },  
                    '@' => {
                        input_string.next();
                        let slice = input.get(ix+1..).ok_or("could not parse")?;
                        let (ident, id_length) = tok_identifier(slice)?;
                        for _ in 0..(id_length-1) {
                            input_string.next();
                        }
                        Some(token_list.push(ident))
                    },
                    '='|'('|')'|'['|']'|','
                       |'.'|'#'|'~'|'<'|'>'
                       |'+'|'-'|'\''|'/'|'\\'
                       |'`'|'|'|'?'|'!'|'$'
                       |'&'|'"'|';'|':'|'*'
                       |'%'|'^'|'{'|'}' => {
                        match char_to_token(&s) {
                            Some(t) => Some(token_list.push(Token {token_type: t})),
                            None => break None
                        }
                    },
                    _ => {
                        bad_char_index = (ix as u8) - last_total_chars;
                        break None
                    }
                };
                input_string.next();
            },
            None => break Some(())
        };
    };
    match res {
        Some(_) => Ok(token_list),
        None => Err(String::from(format!("could not parse at line: {}, char: {}", line_num, bad_char_index)))
    }
    // Err(string) => return Err(String::from(string))
}

// fn tokenize_identifier(input: std::iter::Peekable<std::string::Chars>) -> Result<Token, String> {

// }

// struct ParseTree {}
// #[derive(Debug)]
// enum OperatorExpr {
//     RhythmApply(Token),
// }
#[derive(Debug)]
enum AtomicExpr {
    Unit(Token, Option<TokenType>),
    Identifier(TokenType),
    Operator(TokenType)
}

#[derive(Debug)]
enum SeriesExpr {
    Line(Vec<AtomicExpr>),
    WithRhythm(Box<RhythmExpr>, Box<AtomicExpr>, Box<SeriesExpr>)
}

#[derive(Debug)]
enum ParallelExpr {
    Chord(Vec<AtomicExpr>)
}

#[derive(Debug)]
enum RhythmExpr {
    BeatGrouping(Vec<AtomicExpr>),
    BarGrouping(Vec<AtomicExpr>)
}

#[derive(Debug)]
enum Expr {
    SeriesExpr(SeriesExpr),
    ParallelExpr(ParallelExpr),
    RhythmExpr(RhythmExpr),
    AssignmentExpr(AssignmentExpr),
}

#[derive(Debug)]
enum AssignmentExpr {
    RhythmAssignment(TokenType, Box<RhythmExpr>),
    ParallelAssignment(TokenType, Box<ParallelExpr>),
    SeriesAssignment(TokenType, Box<SeriesExpr>)
}

#[derive(Debug)]
enum Score {
    ExprList(Vec<Expr>)
}

#[derive(Debug)]
struct Parser {
    token_list: TokenList,
    index: usize,
}

#[derive(Debug)]
struct ParseError {
    reason: String,
    at_token: Option<Token>
}

type ParseResult<T> = Result<T, ParseError>;

fn parse_error<T>(reason: String, at_token: Option<Token>) -> ParseResult<T> {
    Err(ParseError{ reason: reason, at_token: at_token})
}

impl Parser {
    fn parse(&self) -> ParseResult<Score> {
        self.parse_score()
    }

    fn parse_score(&self) -> ParseResult<Score> {
        let mut expr_list: Vec<Expr> = vec![];
        loop {
            match self.token_peek() {
                Some(token) => match self.parse_expr() {
                    Ok(expr) => expr_list.push(expr),
                    Err(error) => break Err(error)
                },
                None => break Ok(Score::ExprList(expr_list))
            }
        }
    }

    // SeriesExpr(SeriesExpr),
    // ParallelExpr(ParallelExpr),
    // RhythmExpr(RhythmExpr),
    // RhythmisedSeriesExpr(SeriesExpr, RhythmExpr),
    // AssignmentExpr(Token, Box<Expr>),

    fn parse_expr(&self) -> ParseResult<Expr> {
        match self.token_peek() {
            Some(Token{token_type:token}) => match token {
                TokenType::Tilde => {
                    self.token_advance();
                    match self.parse_series_expr() {
                        Ok(parsed) => Ok(Expr::SeriesExpr(parsed)),
                        Err(e) => Err(e)
                    }
                },
                TokenType::Hash => {
                    self.token_advance();
                    match self.parse_parallel_expr() {
                        Ok(parsed) => Ok(Expr::ParallelExpr(parsed)),
                        Err(e) => Err(e)
                    }
                },
                TokenType::ForwardSlash => {
                    self.token_advance();
                    match self.parse_rhythm_expr() {
                        Ok(parsed) => Ok(Expr::RhythmExpr(parsed)),
                        Err(e) => Err(e)
                    }
                },
                TokenType::Identifier(_) => {
                    match self.parse_assignment_expr() {
                        Ok(parsed) => Ok(Expr::AssignmentExpr(parsed)),
                        Err(e) => Err(e)
                    }
                },
                t => parse_error(String::from("could not parse expression: could not recognise token"), Some(Token{token_type:*t}))
            },
            None => parse_error(String::from("could not parse expression: expected tokens remaining, but token stream is empty"), None)
        }
    }

    fn parse_rhythm_assignment(&self) -> ParseResult<AssignmentExpr> {
        match self.token_peek() {
            Some(Token{token_type:token}) => match token {
                TokenType::Identifier(ident) => {
                    self.token_advance();
                    match self.parse_rhythm_expr() {
                        Ok(par) => Ok(AssignmentExpr::RhythmAssignment(*token,Box::new(par))),
                        Err(err) => Err(err)
                    }
                },
                _ => parse_error(String::from("could not parse rhythm assignment expression: expected identifier"), Some(Token{token_type:*token}))
            },
            None => parse_error(String::from("could not parse rhythm assignment expression: no tokens remaining"), None), 
        }
    }

    fn parse_series_assignment(&self) -> ParseResult<AssignmentExpr> {
        match self.token_peek() {
            Some(Token{token_type:token}) => match token {
                TokenType::Identifier(ident) => {
                    self.token_advance();
                    match self.parse_series_expr() {
                        Ok(par) => Ok(AssignmentExpr::SeriesAssignment(*token,Box::new(par))),
                        Err(err) => Err(err)
                    }
                },
                _ => parse_error(String::from("could not parse series assignment expression: expected identifier"), Some(Token{token_type:*token}))
            },
            None => parse_error(String::from("could not parse series assignment expression: no tokens remaining"), None), 
        }
    }
    
    fn parse_parallel_assignment(&self) -> ParseResult<AssignmentExpr> {
        match self.token_peek() {
            Some(Token{token_type:token}) => match token {
                TokenType::Identifier(ident) => {
                    self.token_advance();
                    match self.parse_parallel_expr() {
                        Ok(par) => Ok(AssignmentExpr::ParallelAssignment(*token,Box::new(par))),
                        Err(err) => Err(err)
                    }
                },
                _ => parse_error(String::from("could not parse parallel assignment expression: expected identifier"), Some(Token{token_type:*token}))
            },
            None => parse_error(String::from("could not parse parallel assignment expression: no tokens remaining"), None), 
        }
    }    

    fn parse_assignment_expr(&self) -> ParseResult<AssignmentExpr> {
        
    }

    fn parse_duration(&self) -> ParseResult<RhythmExpr> {

    }

    fn parse_beat_grouping(&self) -> ParseResult<RhythmExpr> {

    }

    fn parse_bar_grouping(&self) -> ParseResult<RhythmExpr> {
        
    }

    fn parse_rhythm_expr(&self) -> ParseResult<RhythmExpr> {

    }

    fn parse_atomic_expr(&self) -> ParseResult<AtomicExpr> {
        let tk = self.token_peek();
        match tk {
            Some(Token{token_type:token}) => match token {
                TokenType::Identifier(_) => Ok(AtomicExpr::Identifier(*token)),
                TokenType::(_) => Ok(AtomicExpr::Modifier(*token)),
                TokenType::Identifier(_) => Ok(AtomicExpr::Identifier(*token)),
            }
        }
    }

    fn parse_parallel_expr(&self) -> ParseResult<ParallelExpr> {
        let mut chord: Vec<AtomicExpr> = Vec::new();
        let result = loop {
            let tk = self.token_peek();
            match tk {
                Some(Token{token_type:token}) => match token {
                    TokenType::Identifier(a) => {
                        chord.push(AtomicExpr::Identifier(*token))
                    }
                }
            }
        };

        if chord.len() > 0 {
            Ok(ParallelExpr::Chord(chord))
        } else {
            Err(e) => parse_error(String::from("could not parse parallel expression: no parallel subexpressions found"), Some(tk))
        }
    }

    fn parse_series_expr(&self) -> ParseResult<SeriesExpr> {

    }

    fn token_check(&self, token_type: &TokenType) -> bool {
        match self.token_peek() {
            Some(Token { token_type: ttype}) => {
                *ttype == *token_type
            },
            None => false
        }
    }

    fn token_advance(&self) -> Option<&Token> {
        match self.token_list.get(self.index) {
            Some(token) => {
                self.index += 1;
                Some(token)
            },
            None => None
        }
    }

    // fn is_end(&self) -> bool {

    // }

    fn token_peek(&self) -> Option<&Token> {
        self.token_list.get(self.index)
    }

    fn token_previous(&self) -> Option<&Token> {
        self.token_list.get(self.index - 1)
    }


    fn token_match(&self, token_types: Vec<TokenType>) -> Option<&Token> {
        match token_types.iter().skip_while(|tt| self.token_check(tt)).next() {
            Some(tok) => self.token_advance(),
            None => None
        }
    }
}





#[wasm_bindgen]
pub fn generate_parse_tree(input: String) {
    utils::set_panic_hook();
    // tokenize input
    // feed token stream into parse function
    let token_stream = tokenize(&input);
    match token_stream {
        Ok(stream) => console::log_1(&format!("{:#?}", stream).into()),
        Err(error) => console::log_1(&error.into())
    }
    
}


// // impl Iterator for Tokenizer {
// //     type Item = Token;

// //     fn next(&mut self) -> Option<Self::Item> {

// //         self.token_list.push(TokenType::EQUALS);
// //         Some(Token{token_type: TokenType::EQUALS})
// //     }
// // }
// /* parser */
// /* interpreter */