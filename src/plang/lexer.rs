use std::collections::HashSet;
use super::tokenizer::{ Token, TokenType, Tokenizer };
use lazy_static::*;
use std::fmt;

macro_rules! set {
    ( $($x:expr),* ) => {
        {
            let mut temp_set = HashSet::new();
            $(temp_set.insert($x);)*
            temp_set
        }
    };
}

lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = set![
        "program", "program_vege", "eljaras", "eljaras_vege", 
        "fuggveny", "fuggveny_vege", "valtozok", "ha", "akkor", 
        "ha_vege", "kulonben", "ciklus", "amig", "ciklus_vege", "egesz",
        "valos", "szoveg", "karakter", "logikai", "igaz", "hamis",
        "sv", "ki", "be", "sin", "cos", "tan", "log", "exp", "rnd",
        "arcsin", "arccos", "arctan", "kis", "nagy", "kerek", "betu", 
        "szam", "nem", "es", "vagy", "kifajl", "befajl", "megnyit", 
        "lezar", "vege"
    ];

    static ref OPERATORS: HashSet<&'static str> = set![
        ",", ":", "+", "-", "*", "/", "^", "@",
        "div", "mod", "(", ")", "[", "]", "|", ":=", "=", "/=", 
        "<", ">", "<=", ">="
    ];
}


#[derive(PartialEq)]
pub struct LexerToken<'a> {
    pub tok: Token<'a>,
    pub lex: String
}

impl<'a> LexerToken<'a> {
    #[cfg(test)]
    pub fn from(sval: &str, nval: f64, ttype: TokenType) -> LexerToken {
        let lex = if ttype == TokenType::Keyword {
            Lexer::deaccent(sval.to_lowercase())
        } else {
            String::new()
        };

        LexerToken { tok: Token { sval, nval, ttype}, lex }
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    tok: Tokenizer<'a>,
    
    current: Token<'a>,
    current_lex: String
}

impl<'a> Lexer<'a> {
    pub fn from(s: &'a str) -> Self {
        let options = Tokenizer::EOL_MATTERS | Tokenizer::CONCAT_SPECIAL;
        let comment_defs = vec![("**", "")];
        let mut tok = Tokenizer::with_comments(s, options, comment_defs);

        let tok_it = tok.next();
        let current = match tok_it {
            Some(t) => t,
            None => Token { 
                sval: "", 
                nval: 0.0, 
                ttype: TokenType::Undefined 
            } 
        };

        Self { tok, current, current_lex: String::new() }
    }

    fn deaccent<S>(word: S) -> String where S: Into<String>
    {
        word.into().chars().map(|c| match c {
            'á' => 'a',
            'í' => 'i',
            'é' => 'e',
            'ó'|'ö'|'ő' => 'o',
            'ú'|'ü'|'ű' => 'u',
            _ => c
        }).collect()
    }

    fn step(&mut self) {
        let tk_it = self.tok.next();

        if tk_it.is_none() {
            self.current.ttype = TokenType::EOF;
        }

        let mut tk = tk_it.unwrap();
        self.current_lex = match tk.ttype {
            TokenType::Special => {
                if OPERATORS.contains(tk.sval) {
                    tk.ttype = TokenType::Operator;
                    // tk.ttype = TokenType::Keyword;
                } else {
                    tk.ttype = TokenType::Undefined;
                }
                String::new()
            },
            TokenType::Word => {
                let word  = Lexer::deaccent(tk.sval.to_lowercase());
                if KEYWORDS.contains(word.as_str()) {
                    tk.ttype = TokenType::Keyword;
                    word
                } else {
                    tk.ttype = TokenType::Ident;
                    String::new()
                }
            },
            _ => String::new()
        };

        self.current = tk;
    }

    pub fn is_data(&self) -> bool {
        match self.current.ttype {
            TokenType::Char | TokenType::String | 
            TokenType::Int  | TokenType::Float => true,

            _ => false
        } 
    }

    pub fn is_ident(&self) -> bool {
        return self.current.ttype == TokenType::Ident;
    }

    pub fn is_keyword(&self) -> bool {
        return self.current.ttype == TokenType::Keyword;
    }

    pub fn sval(&self) -> &'a str {
        return self.current.sval;
    }

    pub fn nval(&self) -> f64 {
        return self.current.nval;
    }

    pub fn fval(&self) -> f64 {
        return self.current.nval;
    }

    pub fn ival(&self) -> i64 {
        return self.current.nval as i64;
    }

    pub fn lexical(&self) -> String {
        return self.current_lex.clone();
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexerToken<'a>;

    fn next(&mut self) -> Option<LexerToken<'a>> {
        let rv = match self.current.ttype {
            TokenType::EOF => Option::from(None),
            _ => Option::from(LexerToken { 
                tok: self.current.clone(), lex: self.current_lex.clone()
            })
        };
        self.step();
        rv
    }
}

impl<'a> fmt::Debug for LexerToken<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.tok.ttype {
            TokenType::Word => {
                f.debug_struct("LexerToken")
                .field("sval", &self.tok.sval)
                .field("nval", &self.tok.nval)
                .field("type", &self.tok.ttype)
                .field("lexical", &self.lex)
                .finish()
            } 
            _ =>  {
                f.debug_struct("LexerToken")
                .field("sval", &self.tok.sval)
                .field("nval", &self.tok.nval)
                .field("type", &self.tok.ttype)
                .finish()
            }
        }
    }
}