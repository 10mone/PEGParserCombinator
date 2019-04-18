extern crate peg_parser_combinator;


#[cfg(test)]
mod exp{
    use std::collections::HashMap;
    use peg_parser_combinator::{parser_context::parser_context::ParserContext, exp::exp::Exp};
    use peg_parser_combinator::exp::exp::*;


    #[test]
    fn anychar1() {
        let s: &'static str = "a";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),HashMap::new());
        let peg: Exp = any();
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }

    #[test]
    fn anychar2() {
        let s: &'static str = "aaa";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),HashMap::new());
        let peg: Exp = seq(any(),seq(any(),any()));
//      let peg: Exp =  Exp::Seq{e1: Box::new(Exp::AnyChar), e2: Box::new(Exp::Seq{e1: Box::new(Exp::AnyChar), e2: Box::new(Exp::AnyChar)})};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }

    #[test]
    fn char1() {
        let s: &'static str = "a";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),HashMap::new());
        let peg: Exp = Exp::Char{c: 'a'};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }

    #[test]
    fn char2() {
        let s: &'static str = "aaa";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),HashMap::new());
        let peg: Exp = Exp::Seq{e1: Box::new(Exp::Char{c: 'a'}), e2: Box::new(Exp::Seq{e1: Box::new(Exp::Char{c: 'a'}), e2: Box::new(Exp::Char{c: 'a'})})};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }
    #[test]
    fn choice1() {
        let s: &'static str = "a";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),HashMap::new());
        let peg: Exp = Exp::Choice{e1: Box::new(Exp::Char{c: 'a'}),e2: Box::new(Exp::Char{c: 'b'})};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }
    #[test]
    fn choice2() {
        let s: &'static str = "b";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),HashMap::new());
        let peg: Exp = Exp::Choice{e1: Box::new(Exp::Char{c: 'a'}),e2: Box::new(Exp::Char{c: 'b'})};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }
    #[test]
    fn zero_or_more1() {
        let s: &'static str = "";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),HashMap::new());
        let peg: Exp = Exp::Rep{e: Box::new(Exp::Char{c: 'a'})};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }
    #[test]
    fn zero_or_more2() {
        let s: &'static str = "aaaaaaaaaaaaaaaaaaaa";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),HashMap::new());
        let peg: Exp = Exp::Rep{e: Box::new(Exp::Char{c: 'a'})};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }
    #[test]
    fn one_or_more1() {
        let s: &'static str = "a";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),HashMap::new());
        let peg: Exp = Exp::Seq{e1: Box::new(Exp::Char{c: 'a'}),e2: Box::new(Exp::Rep{e: Box::new(Exp::Char{c: 'a'})})};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }
    #[test]
    fn one_or_more2() {
        let s: &'static str = "aaaaaaaaaaaaaaaaaaa";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),HashMap::new());
        let peg: Exp = Exp::Seq{e1: Box::new(Exp::Char{c: 'a'}),e2: Box::new(Exp::Rep{e: Box::new(Exp::Char{c: 'a'})})};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }
    #[test]
    fn opt1() {
        let s: &'static str = "";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),HashMap::new());
        let peg: Exp = Exp::Opt{e: Box::new(Exp::Char{c: 'a'})};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }
    #[test]
    fn opt2() {
        let s: &'static str = "a";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),HashMap::new());
        let peg: Exp = Exp::Opt{e: Box::new(Exp::Char{c: 'a'})};
        assert!(peg.parse(&mut p) && (p.pos == s.len()));
    }
    #[test]
    fn not1() {
        let s: &'static str = "b";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),HashMap::new());
        let peg: Exp = Exp::Not{e: Box::new(Exp::Char{c: 'a'})};
        assert!(peg.parse(&mut p) && (p.pos == 0));
    }
    #[test]
    fn and1() {
        let s: &'static str = "a";
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),HashMap::new());
        let peg: Exp = Exp::Not{e: Box::new(Exp::Not{e: Box::new(Exp::Char{c: 'a'})})};
        assert!(peg.parse(&mut p) && (p.pos == 0));
    }
    #[test]
    fn symbol1() {
        let s: &'static str = "ab";
        let mut rules = HashMap::new();
        rules.insert("A",Exp::Seq{e1: Box::new(Exp::Char{c: 'a'}),e2: Box::new(Exp::Symbol{sym: "B"})});
        rules.insert("B",Exp::Char{c: 'b'});
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),rules.clone());
        assert!(rules.get(&"A").unwrap().parse(&mut p) && (p.pos == s.len()));
    }
     #[test]
    fn math1() {
        let s: &'static str = "3-2-1";
        let mut rules = HashMap::new();
        rules.insert("EXP",seq(sym("PRODUCT"),rep0(choice(seq(char('+'),sym("PRODUCT")),seq(char('-'),sym("PRODUCT"))))));
        rules.insert("PRODUCT",seq(sym("NUM"),rep0(choice(seq(char('*'),sym("NUM")),seq(char('/'),sym("NUM"))))));
        rules.insert("NUM",rep1(choice(char('0'),choice(char('1'),choice(char('2'),choice(char('3'),choice(char('4'),choice(char('5'),choice(char('6'),choice(char('7'),choice(char('8'),char('9'))))))))))));
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),rules.clone());
        assert!(rules.get(&"EXP").unwrap().parse(&mut p) && (p.pos == s.len()));
    }
     #[test]
    fn math2() {
        let s: &'static str = "3+2+1";
        let mut rules = HashMap::new();
        rules.insert("EXP",seq(sym("PRODUCT"),rep0(choice(seq(char('+'),sym("PRODUCT")),seq(char('-'),sym("PRODUCT"))))));
        rules.insert("PRODUCT",seq(sym("NUM"),rep0(choice(seq(char('*'),sym("NUM")),seq(char('/'),sym("NUM"))))));
        rules.insert("NUM",rep1(choice(char('0'),choice(char('1'),choice(char('2'),choice(char('3'),choice(char('4'),choice(char('5'),choice(char('6'),choice(char('7'),choice(char('8'),char('9'))))))))))));
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),rules.clone());
        assert!(Exp::Symbol{sym: &"EXP"}.parse(&mut p) && (p.pos == s.len()));
        assert!(p.tree[0].two_string() == "[EXP [PRODUCT [NUM 3]] + [PRODUCT [NUM 2]] + [PRODUCT [NUM 1]]]")
    }

    #[test]
    fn tree() {
        let s: &'static str = "abc";
        let mut rules = HashMap::new();
        rules.insert("S",seq(sym("A"),sym("BC")));
        rules.insert("A",char('a'));
        rules.insert("BC",seq(char('b'),char('c')));  
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),rules.clone());
        Exp::Symbol{sym: &"S"}.parse(&mut p);
        assert!(p.tree[0].two_string() == "[S [A a] [BC b c]]");
    }

    #[test]
    fn cabcat() {
        let s: &'static str = "cat";
        let mut rules = HashMap::new();
        rules.insert("S",choice(sym("CAB"),sym("CAT")));
        rules.insert("CAB",seq(char('c'),seq(char('a'),char('b'))));
        rules.insert("CAT",seq(char('c'),seq(char('a'),char('t'))));
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),rules.clone());
        Exp::Symbol{sym: &"S"}.parse(&mut p);
        assert!(p.tree[0].two_string() == "[S [CAT c a t]]")
    }

    #[test]
    fn hayasa() {
        let s: &'static str = "(((((((1)))))))";
        let mut rules = HashMap::new();
        rules.insert("P",choice(seq(char('('),seq(sym("A"),char(')'))),char('1')));
        rules.insert("A",choice(seq(sym("P"),seq(char('+'),sym("A"))),choice(seq(sym("P"),seq(char('-'),sym("A"))),sym("P"))));
        let mut p: ParserContext = ParserContext::new(String::from(s).into_bytes(),rules.clone());
        assert!(rules.get(&"A").unwrap().parse(&mut p) && (p.pos == s.len()));
 
    }

}