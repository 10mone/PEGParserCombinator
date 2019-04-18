extern crate peg_parser_combinator;

#[cfg(test)]
mod exp{
    use peg_parser_combinator::{parser_context::parser_context::ParserContext};
    use peg_parser_combinator::combinator::combinator::*;
    use peg_parser_combinator::charset::charset::*;

    const symbol : [&'static str; 4] = ["S","A","B","C"];

    #[test]
    fn anychar1() {
        let s: &'static str = "a";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
            vec![
                anychar()
            ]
        );
        assert!(p.rules[0](&p) && (p.pos.get() == s.len()));
    }

    #[test]
    fn anychar2() {
        let s: &'static str = "aaa";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
            vec![
                seq(anychar(),seq(anychar(),anychar()))
            ]
        );
        assert!(p.rules[0](&p) && (p.pos.get() == s.len()));
    }

    #[test]
    fn char1() {
        let s: &'static str = "a";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
            vec![
                char('a')
            ]
        );
        assert!(p.rules[0](&p) && (p.pos.get() == s.len()));
    }

    #[test]
    fn char2() {
        let s: &'static str = "aaa";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
            vec![
                seq(char('a'),seq(char('a'),char('a')))
            ]
        );
        assert!(p.rules[0](&p) && (p.pos.get() == s.len()));
    }
    #[test]
    fn choice1() {
        let s: &'static str = "a";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
            vec![
                choice(char('a'),char('b'))
            ]
        );
        assert!(p.rules[0](&p) && (p.pos.get() == s.len()));
    }
    #[test]
    fn choice2() {
        let s: &'static str = "b";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
            vec![
                choice(char('a'),char('b'))
            ]
        );
        assert!(p.rules[0](&p) && (p.pos.get() == s.len()));
    }
    #[test]
    fn zero_or_more1() {
        let s: &'static str = "";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
            vec![
                rep(char('a'))
            ]
        );
        assert!(p.rules[0](&p) && (p.pos.get() == s.len()));
    }
    #[test]
    fn zero_or_more2() {
        let s: &'static str = "aaaaaaaaaaaaaaaaaaaa";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
            vec![
                rep(char('a'))
            ]
        );
        assert!(p.rules[0](&p) && (p.pos.get() == s.len()));
    }
    #[test]
    fn one_or_more1() {
        let s: &'static str = "a";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
            vec![
                seq(char('a'),rep(char('a')))
            ]
        );
        assert!(p.rules[0](&p) && (p.pos.get() == s.len()));
    }
    #[test]
    fn one_or_more2() {
        let s: &'static str = "aaaaaaaaaaaaaaaaaaa";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
            vec![
                seq(char('a'),rep(char('a')))
            ]
        );
        assert!(p.rules[0](&p) && (p.pos.get() == s.len()));
    }
    #[test]
    fn opt1() {
        let s: &'static str = "";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
            vec![
                opt(char('a'))
            ]
        );
        assert!(p.rules[0](&p) && (p.pos.get() == s.len()));
    }
    #[test]
    fn opt2() {
        let s: &'static str = "a";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
            vec![
                opt(char('a'))
            ]
        );
        assert!(p.rules[0](&p) && (p.pos.get() == s.len()));
    }
    #[test]
    fn not1() {
        let s: &'static str = "b";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
            vec![
                not(char('a'))
            ]
        );
        assert!(p.rules[0](&p) && (p.pos.get() == 0));
    }
    #[test]
    fn and1() {
        let s: &'static str = "a";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
            vec![
                not(not(char('a')))
            ]
        );
        assert!(p.rules[0](&p) && (p.pos.get() == 0));
    }
    #[test]
    fn symbol1() {
        let s: &'static str = "ab";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
            vec![
                seq(char('a'),nonterminal(1)),
                char('b')
            ]
        );
        assert!(p.rules[0](&p) && (p.pos.get() == s.len()));
    }
     #[test]
    fn math1() {
        let s: &'static str = "3-2-1";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
                vec![
                    seq(nonterminal(1),rep(choice(seq(char('+'),nonterminal(1)),seq(char('-'),nonterminal(1))))),
                    seq(nonterminal(2),rep(choice(seq(char('*'),nonterminal(2)),seq(char('/'),nonterminal(2))))),
                    seq(nonterminal(3),rep(nonterminal(3))),
                    //choice(char('0'),choice(char('1'),choice(char('2'),choice(char('3'),choice(char('4'),choice(char('5'),choice(char('6'),choice(char('7'),choice(char('8'),char('9'))))))))))
                    range('0','9')
                ]
            );
        assert!(nonterminal(0)(&p) && (p.pos.get() == s.len()));
        //assert!(p.tree.borrow()[0].two_string(&symbol) == "[S [A [B [C 3]]] - [A [B [C 2]]] - [A [B [C 1]]]]");
    }
     #[test]
    fn math2() {
        let s: &'static str = "9+2+0";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
                vec![
                    seq(nonterminal(1),rep(choice(seq(char('+'),nonterminal(1)),seq(char('-'),nonterminal(1))))),
                    seq(nonterminal(2),rep(choice(seq(char('*'),nonterminal(2)),seq(char('/'),nonterminal(2))))),
                    seq(nonterminal(3),rep(nonterminal(3))),
                    range('0','9')
                ]
        );
        assert!(nonterminal(0)(&p) && (p.pos.get() == s.len()));
        //assert!(p.tree.borrow()[0].two_string(&symbol) == "[S [A [B [C 9]]] + [A [B [C 2]]] + [A [B [C 0]]]]");
    }

    #[test]
    fn tree() {
        let s: &'static str = "abc";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
            vec![
                seq(nonterminal(1),nonterminal(2)),
                char('a'),
                seq(char('b'),char('c'))
            ]
        );
        assert!(nonterminal(0)(&p)&& (p.pos.get() == s.len()));
        //assert!(p.tree.borrow()[0].two_string(&symbol) == "[S [A a] [B b c]]");
        
    }

    #[test]
    fn cabcat() {
        let s: &'static str = "cat";        
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
            vec![
                choice(nonterminal(1),nonterminal(2)),
                seq(char('c'),seq(char('a'),char('b'))),
                seq(char('c'),seq(char('a'),char('t')))
            ]
        );
        assert!(nonterminal(0)(&p) && (p.pos.get() == s.len()));
        //assert!(p.tree.borrow()[0].two_string(&symbol) == "[S [B c a t]]");
    }
#[macro_use(charset)]
    #[test]
    fn hayasa() {
        let s: &'static str = "(((((((1)))))))";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
            vec![
                //choice(seq(char('('),seq(nonterminal(1),char(')'))),char('1')),
                (char('(') - nonterminal(1) - char(')')) | (char('1')),
                //choice(seq(nonterminal(0),seq(char('+'),nonterminal(1))),choice(seq(nonterminal(0),seq(char('-'),nonterminal(1))),nonterminal(0)))
                (nonterminal(0) - char('+') - nonterminal(1)) | ((nonterminal(0) - char('-') - nonterminal(1)) | (nonterminal(0)))
                //charset![ch(nonterminal(0) - char('+') - nonterminal(1)) , ch(nonterminal(0) - char('-') - nonterminal(1)) , ch(nonterminal(0))]
            ]
        );
        assert!(nonterminal(0)(&p) && (p.pos.get() == s.len()));
        //assert!(p.tree.borrow()[0].two_string(&symbol) == "[S ( [A [S ( [A [S ( [A [S ( [A [S ( [A [S ( [A [S ( [A [S 1]] )]] )]] )]] )]] )]] )]] )]");
 
    }
        #[test]
    fn stringtest() {
        let s: &'static str = "tomone";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
            vec![
                //seq(nonterminal(1),seq(nonterminal(2),nonterminal(3))),
                nonterminal(1) - nonterminal(2) - nonterminal(3),
                char('t'),
                string("omo"),
                //seq(char('n'),char('e'))
                char('n') - char('e')
            ]
        );
        assert!(nonterminal(0)(&p) && (p.pos.get() == s.len()));
        //assert!(p.tree.borrow()[0].two_string(&symbol) == "[S [A t] [B omo] [C n e]]");
    }

    #[test]
    fn comment() {
        let s: &'static str = "<!-- aggregated reports for multi-module projects -->     ";
        let p: ParserContext = ParserContext::new(
            String::from(s).into_bytes(),
            vec![
                string("<!--") - rep(not(string("-->")) - anychar()) - string("-->") - rep(nonterminal(1)),
                char(' ') | string("\t") | string("\r") | string("\n"),
            ]
        );
        assert!(nonterminal(0)(&p) && (p.pos.get() == s.len()));
        //assert!(p.tree.borrow()[0].two_string(&symbol) == "[S [A t] [B omo] [C n e]]");
    }
}