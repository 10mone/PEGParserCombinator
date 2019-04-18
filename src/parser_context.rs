pub mod parser_context{
    use tree::tree::Tree;
    use combinator::combinator;
    use combinator::combinator::Combinator;
    use memo::memo::Memos;
    use treelog::treelog::TreeLog;
    use std::cell::Cell;
    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct ParserContext{
        pub input: Vec<u8>, //バイト配列
        pub input_len: usize,
        pub pos: Cell<usize>,
        pub tree: RefCell<ContextTree>,
        pub rules: Vec<Combinator>,   
        pub memos: RefCell<Vec<Vec<Memos>>>,
    }
    #[derive(Debug,Clone)]
    pub struct ContextTree{
        pub tree: Rc<TreeLog>,
    }
    impl ContextTree{
        pub fn set(&mut self, log: Rc<TreeLog>){
            self.tree = log;
        }
        pub fn log2tree(&self) -> Rc<Vec<Tree>>{
            self.tree.log2tree()
        }
    }
    impl ParserContext{
        pub fn new(input: Vec<u8>,rules: Vec<Combinator>) -> ParserContext{
            ParserContext{
                memos: RefCell::new(Self::new_memo(256, rules.len())),
                input_len: input.len(),
                input: input,
                pos: Cell::new(0),
                tree: RefCell::new(ContextTree{tree: Rc::new(TreeLog::Nil)}),
                rules: rules,
            }
        }
        fn new_memo(input_size:usize ,rules_size:usize) -> Vec<Vec<Memos>> {
            let mut table = Vec::new();
            for _ in 0..rules_size{
                let mut positions = Vec::new();
                for _ in 0..input_size{
                    positions.push(Memos::Nil);
                }
                table.push(positions);
            }
            table
        }
        pub fn lookup(&self, pos: usize, sym: usize) -> Memos {
            self.memos.borrow()[sym][pos % 256].clone()
        }
        pub fn succ_memo(&self, old_pos: usize, sym: usize, new_pos: usize ,child: Rc<TreeLog>) {
            self.memos.borrow_mut()[sym][old_pos % 256] = Memos::Memo{pos: new_pos,oldpos: old_pos,child: child};
        }
        pub fn fail_memo(&self, old_pos: usize, sym: usize) {
            self.memos.borrow_mut()[sym][old_pos % 256] = Memos::Fail{oldpos: old_pos}; 
        }
        
    }
}