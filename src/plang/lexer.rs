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
    pub fn is_data(&self) -> bool {
        match self.tok.ttype {
            TokenType::Char | TokenType::String | 
            TokenType::Int  | TokenType::Float => true,

            _ => false
        } 
    }

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
}

impl<'a> Lexer<'a> {
    pub fn from(s: &'a str) -> Self {
        let options = Tokenizer::EOL_MATTERS | Tokenizer::CONCAT_SPECIAL;
        let comment_defs = vec![("**", "")];
        let tok = Tokenizer::with_comments(s, options, comment_defs);

        Self { tok }
    }

    fn deaccent<S>(word: S) -> String where S: Into<String>
    {
        word
        .into()
        .chars()
        .map(|c| match c {
            'á' => 'a',
            'í' => 'i',
            'é' => 'e',
            'ó'|'ö'|'ő' => 'o',
            'ú'|'ü'|'ű' => 'u',
            _ => c
        }).collect()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexerToken<'a>;

    fn next(&mut self) -> Option<LexerToken<'a>> {
        let tk_it = self.tok.next();

        if tk_it.is_none() {
            return Option::from(None);
        }

        let mut tk = tk_it.unwrap();
        let lex = match tk.ttype {
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

        let lt = LexerToken { tok: tk, lex };
        return Option::from(lt);
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