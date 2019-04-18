extern crate peg_parser_combinator;

use std::collections::HashMap;
use peg_parser_combinator::{parser_context::parser_context::ParserContext, exp::exp::Exp,tree::tree::Tree};
use peg_parser_combinator::exp::exp::*;


fn main() {
    let s: &'static str = "((((((((((1))))))))))";
    let mut rules = HashMap::new();
    rules.insert("P",choice(seq(char('('),seq(sym("A"),char(')'))),char('1')));
    rules.insert("A",choice(seq(sym("P"),seq(char('+'),sym("A"))),choice(seq(sym("P"),seq(char('-'),sym("A"))),sym("P"))));
    let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),rules.clone());
    Exp::Symbol{sym:&"A"}.parse(&mut p);

    println!("{}",p.tree[0].two_string());   
}


