mod plang;
use std::collections::HashMap;
use plang::{ Tokenizer, TokenType, Lexer, LexerToken }; 
use itertools::Itertools;

fn main() {
    let script = r#"** this is a commentá
        ** so 12312 is :=== this
        PROGRÁM a

        VALTOZOK:
        x: SZOVEG
        k: KARAKTER
        e1: EGESZ

        x:=" vmi szöveg \u644  ...ja"
        e1 :=123.0.0,5
        k := "a"
        c :='a'
        _asd :== 123213

        ___a__\u6653

        i32 := 1024 

        PROGRÁM_VÉGE
    "#;

    let lx = Lexer::from(script);

    // for t in lx.into_iter().filter(|t| t.tok.ttype == TokenType::Undefined) {
    //     println!("{:?}", t);
    // }

    for t in lx {
        println!("{:?}", t);
    }
}
