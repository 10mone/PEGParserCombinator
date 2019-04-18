
#[macro_use(charset)]
extern crate peg_parser_combinator;
use peg_parser_combinator::{parser_context::parser_context::ParserContext, combinator::combinator,tree::tree::Tree};
use peg_parser_combinator::combinator::combinator::*;
use peg_parser_combinator::charset::charset::*;
use std::time::{Instant};
use std::env;
use std::{fs, mem};
use std::io::{BufReader, Read};
use std::fs::File;
use std::io;
use std::io::prelude::*;

macro_rules! measure {
  ( $x:expr) => {
    {
      let start = Instant::now();
      let result = $x;
      let end = start.elapsed();
      println!("{}.{:09}[sec]", end.as_secs(), end.subsec_nanos() / 1_000_000);
      result
    }
  };
}

fn main() {
    let mut f = File::open("xmark10.xml").unwrap();
    //let mut f = File::open("pom.xml").unwrap();
    let mut s = Vec::new();

    // read the whole file
    f.read_to_end(&mut s);



    let p = ParserContext::new(
        s,
        vec![
            opt(nonterminal(3)) - opt(nonterminal(4)) - nonterminal(5),
            nonterminal(5),
            nonterminal(5),
            string("<?xml") - rep(not(string("?>")) - anychar()) - string("?>") - rep(nonterminal(14)),
            string("<!") - rep(not(char('>')) - anychar()) - char('>') - rep(nonterminal(14)),
            char('<') - nonterminal(6) - rep(nonterminal(14)) - rep(nonterminal(7)) - (string("/>") | char('>') - rep(nonterminal(14)) - rep(nonterminal(9) | nonterminal(12)) - string("</") - nonterminal(6) - char('>')) - rep(nonterminal(14)),
            //(char(':') | range('A', 'Z') | char('_') | range('a', 'z')) - rep(char('-') | char('.') | range('0', '9') | char(':') | range('A','Z') | char('_') | range('a', 'z')),
            charset![ch(':'), ch('A') - ch('Z'), ch('_'), ch('a') - ch('z')] - rep(charset![ch('-'), ch('.'), ch('0') - ch('9'), ch(':'), ch('A') - ch('Z'), ch('_'), ch('a') - ch('z')]),
            nonterminal(6) - rep(nonterminal(14)) - char('=') - rep(nonterminal(14)) - nonterminal(8) - rep(nonterminal(14)),
            char('"') - rep(not(char('"')) - anychar()) - char('"'),
            nonterminal(5) | nonterminal(10) | nonterminal(13),
            string("<![CDATA[") - nonterminal(11) - string("]]>") - rep(nonterminal(14)),
            rep(not(string("]]>")) - not(string("<![CDATA[")) - anychar()) - opt(string("<![CDATA[") - nonterminal(11)),
            string("<!--") - rep(not(string("-->")) - anychar()) - string("-->") - rep(nonterminal(14)),
            (not(char('<')) - anychar()) - rep(not(char('<')) - anychar()),
            //char(' ') | char('\t') | char('\r') | char('\n')
            charset![ch(' '), ch('\t'), ch('\r'), ch('\n')]
        ]
    );

    println!("waiting");
    measure!({
        println!("{}",nonterminal(0)(&p));
    });

    let symbol = vec!["File","Chunk","Expr","PROLOG","DTD","Xml","Name","Attribute","String","Content","CDataSec","CDATA","COMMENT","Text","S"];
    //println!("{}",p.tree.borrow()[0].two_string(&symbol)); 

    
}
