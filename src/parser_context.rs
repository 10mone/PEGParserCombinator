pub mod parser_context{
    use std::collections::HashMap;
    use tree::tree::Tree;
    use exp::exp::Exp;
    use memo::memo::Memos;

    #[derive(Debug,Clone)]
    pub struct ParserContext{
        pub input: Vec<u8>, //バイト配列
        pub input_len: usize,
        pub pos: usize,
        pub tree: Vec<Tree>,
        pub rules: HashMap<&'static str,Exp>,
        pub memos: HashMap<(usize,&'static str),Memos>,
    }

    impl ParserContext{
        pub fn new(input: Vec<u8>,rules: HashMap<&'static str,Exp>) -> ParserContext{
            ParserContext{
                input_len: input.len(),
                input: input,
                pos: 0,
                tree: Vec::new(),
                rules: rules,
                memos: HashMap::new(),
            }
        }
    }

}