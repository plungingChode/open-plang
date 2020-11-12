use std::fmt;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum TokenType {
    Undefined,
    Int,
    Float,
    String,
    Char,
    Word,
    Keyword,
    Ident,
    Special,
    Operator,
    LineBreak,
    Comment
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Token<'a> {
    pub sval: &'a str,
    pub nval: f64,
    pub ttype: TokenType
}

impl Token<'_> {
    pub fn from(sval: &str, nval: f64, ttype: TokenType) -> Token {
        Token { sval, nval, ttype }
    }

    pub fn from_str(sval: &str, ttype: TokenType) -> Token {
        Token { sval, nval: 0.0, ttype }
    }
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    cursor: usize,
    text: &'a str,

    /// Set to true if end-of-line is significant.
    /// Default value is `false`.
    eol_matters: bool,

    /// Set to true, if consecutive, non-alphanumeric characters should
    /// count as a single token. 
    /// Default value is `false`.
    concat_special: bool,

    comment_defs: Vec<(&'static str, &'static str)>,
}

impl<'a> Tokenizer<'a> {
    pub const EOL_MATTERS: i32    = 0b00001;
    pub const CONCAT_SPECIAL: i32 = 0b00010;

    pub fn from(s: &'a str) -> Tokenizer<'a>
    {
        Tokenizer::with_options(s, 0)
    }

    pub fn with_options(s: &'a str, options: i32) -> Tokenizer<'a>
    {
        Tokenizer::with_comments(s, options, vec![])
    }

    pub fn with_comments(
        s: &'a str, 
        options: i32, 
        comment_defs: Vec<(&'static str, &'static str)>) -> Tokenizer<'a>
    {
        let eol_matters = (options & Self::EOL_MATTERS) != 0;
        let concat_special = (options & Self::CONCAT_SPECIAL) != 0;   

        Tokenizer { 
            cursor: 0, 
            text: s, 
            eol_matters, 
            concat_special,
            comment_defs
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        let mut chars = self.text[self.cursor..].chars();
        let mut char_it = chars.next();
        
        while char_it.is_some() && char_it.unwrap().is_whitespace() {
            let c = char_it.unwrap();
            self.cursor += c.len_utf8();
            
            if self.eol_matters { 
                let eol = match c {
                    '\r' => {
                        let pk = chars.by_ref()
                            .peekable().peek()
                            .unwrap_or(&'\0')
                            .clone();

                        if pk != '\n' { 
                            "\r" 
                        } else {
                            chars.next();
                            self.cursor += 1;
                            "\r\n"
                        }
                    },
                    '\n' => "\n",
                    _ => ""
                };

                if !eol.is_empty() {
                    let tk = Token::from_str(eol, TokenType::LineBreak);
                    return Option::from(tk);
                }
            }

            char_it = chars.next();
        }
        
        if char_it.is_none() {
            return Option::from(None);
        }

        let mut ttype: TokenType;
        let begin: usize;
        let end: usize;
        let start_char = char_it.unwrap();

        self.cursor += start_char.len_utf8();

        if start_char == '"' || start_char == '\'' {
            begin = self.cursor + start_char.len_utf8();
            let str_marker = start_char;

            for c in chars.by_ref() {
                if c == str_marker { break; }
                self.cursor += c.len_utf8();
            }

            if chars.next().is_none() {
                ttype = TokenType::Undefined;
                end = self.cursor;
            }
            else {
                ttype = if str_marker == '\'' { TokenType::Char } else { TokenType::String };
                end = self.cursor;
                self.cursor += str_marker.len_utf8();
            }
        }
        else if start_char.is_numeric() {
            begin = self.cursor;
            let mut decimal_set = false;
                
            for c in chars {
                if !decimal_set && (c == ',' || c == '.') {
                    decimal_set = true;
                    self.cursor += c.len_utf8();
                    continue;
                }
                if !c.is_numeric() { break; }
                self.cursor += c.len_utf8();
            }

            ttype = if decimal_set { TokenType::Float } else { TokenType::Int };
            end = self.cursor;
        }
        else if start_char.is_alphanumeric()  || start_char == '_' {
            begin = self.cursor;

            for c in chars {
                if c != '_' && !c.is_alphanumeric() { break; }
                self.cursor += c.len_utf8();
            }

            ttype = TokenType::Word;
            end = self.cursor;
        }
        else {
            ttype = TokenType::Special;
            begin = self.cursor;

            for c in chars {
                if c == '\'' || c == '"' ||
                   c.is_alphanumeric() || c == '_' ||
                   c.is_whitespace() 
                {
                    break;
                }
                self.cursor += c.len_utf8();
            }
            end = self.cursor;
        }

        let sval: &str = &self.text[begin - start_char.len_utf8()..end];

        // parse comment
        let def = self.comment_defs.iter().find(|cd| cd.0 == sval);
        if def.is_some() {
            let cmt_end = def.unwrap().1;
            let begin = self.cursor;

            // no ending specified means until line break
            let opt_end = if cmt_end.is_empty() {
                self.text[begin..].find(&['\r', '\n'][..])
            } else {
                self.text[begin..].find(cmt_end)
            };

            let end = match opt_end {
                Some(x) => begin + x,
                None => self.text.len()
            };

            let sval = &self.text[begin..end];
            self.cursor = end + cmt_end.len();
            return Option::from(Token::from(sval, 0.0, TokenType::Comment));
        }

        let mut nval: f64 = 0.0;

        if ttype == TokenType::Char && !sval.len() == 1 {
            ttype = TokenType::Undefined;
        } 
        else if ttype == TokenType::Float {
            nval = sval
                .replace(',', ".")
                .parse::<f64>()
                .unwrap_or(f64::NAN);
            if nval.is_nan() { ttype = TokenType::Undefined }
        } 
        else if ttype == TokenType::Word && 
                !sval.chars().any(char::is_alphanumeric) 
        {
            // only underscores don't count as words
            ttype = TokenType::Undefined;
        } 
        else if ttype == TokenType::Int {
            nval = sval.parse::<i64>().unwrap() as f64;
        }

        return Option::from(Token::from(sval, nval, ttype));
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_vec_eq {
        ($v1:ident, $v2:ident) => {
            assert_eq!($v1.len(), $v2.len());
            for i in 0..$v1.len() {
                assert_eq!(&$v1[i], &$v2[i]);
            }
        };
    }

    #[test]
    fn ignore_linebreak() {
        let s = "aáb\naa汉汉aab\r\n____ aaaab\n123\n456.7";
        let result: Vec<Token> = Tokenizer::from(s).collect();

        let expect = vec![
            Token::from_str("aáb", TokenType::Word),
            Token::from_str("aa汉汉aab", TokenType::Word),
            Token::from_str("____", TokenType::Undefined),
            Token::from_str("aaaab", TokenType::Word),
            Token::from("123", 123.0, TokenType::Int),
            Token::from("456.7", 456.7, TokenType::Float),
        ];

        assert_vec_eq!(result, expect);
    }

    #[test]
    fn with_linebreak() {
        let s = "aáb\naa汉汉aab\r\naaaab\n123\r\r456.7";
        let result: Vec<Token> = Tokenizer::with_options(s, Tokenizer::EOL_MATTERS).collect();

        let expect = vec![
            Token::from_str("aáb", TokenType::Word),
            Token::from_str("\n", TokenType::LineBreak),
            Token::from_str("aa汉汉aab", TokenType::Word),
            Token::from_str("\r\n", TokenType::LineBreak),
            Token::from_str("aaaab", TokenType::Word),
            Token::from_str("\n", TokenType::LineBreak),
            Token::from("123", 123.0, TokenType::Int),
            Token::from_str("\r", TokenType::LineBreak),
            Token::from_str("\r", TokenType::LineBreak),
            Token::from("456.7", 456.7, TokenType::Float),
        ];

        assert_vec_eq!(result, expect);
    }

    #[test]
    fn number_at_eof() {
        let s = "aaaab aaa123.4";
        let result: Vec<Token> = Tokenizer::from(s).collect();

        let expect = vec![
            Token::from_str("aaaab", TokenType::Word),
            Token::from_str("aaa123", TokenType::Word),
            Token::from_str(".", TokenType::Special),
            Token::from("4", 4.0, TokenType::Int),
        ];

        assert_vec_eq!(result, expect);
    }

    #[test]
    fn concat_special() {
        let s = "x := 123; y:=45.6; x <= y";
        let result: Vec<Token> = Tokenizer::with_options(s, Tokenizer::CONCAT_SPECIAL).collect();

        let expect = vec![
            Token::from_str("x", TokenType::Word),
            Token::from_str(":=", TokenType::Special),
            Token::from("123", 123.0, TokenType::Int),
            Token::from_str(";", TokenType::Special),
            Token::from_str("y", TokenType::Word),
            Token::from_str(":=", TokenType::Special),
            Token::from("45.6", 45.6, TokenType::Float),
            Token::from_str(";", TokenType::Special),
            Token::from_str("x", TokenType::Word),
            Token::from_str("<=", TokenType::Special),
            Token::from_str("y", TokenType::Word),
        ];

        assert_vec_eq!(result, expect);
    }

    #[test]
    fn with_comments() {
        let s = "// aab\n//aab \r\n/* aab */";
        let comment_defs = vec![("//", ""), ("/*", "*/")];
        let options = Tokenizer::CONCAT_SPECIAL | Tokenizer::EOL_MATTERS;
        let result: Vec<Token> = Tokenizer::with_comments(s, options, comment_defs).collect();
        
        let expect = vec![
            Token::from_str(" aab", TokenType::Comment),
            Token::from_str("\n", TokenType::LineBreak),
            Token::from_str("aab ", TokenType::Comment),
            Token::from_str("\r\n", TokenType::LineBreak),
            Token::from_str(" aab ", TokenType::Comment),
        ];

        dbg!(&result);
        assert_vec_eq!(result, expect);
    }
}