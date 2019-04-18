pub mod combinator{
    use parser_context::parser_context::ParserContext;
    use tree::tree::Tree;
    use memo::memo::Memos;
    use treelog::treelog::TreeLog;
    use std::ops::Sub;
    use std::ops::Deref;
    use std::ops::BitOr;
    use std::rc::Rc;
    use parser_context::parser_context::ContextTree;


    pub struct Combinator(Box<Fn(& ParserContext) -> bool>);

    impl Deref for Combinator {
        type Target = Box<Fn(& ParserContext) -> bool>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Sub for Combinator {
        type Output = Combinator;

        fn sub(self, other: Combinator) -> Combinator {
            seq(self, other)
        }
    }

    impl BitOr for Combinator {
        type Output = Combinator;

        fn bitor(self, rhs: Combinator) -> Combinator {
            choice(self, rhs)
        }
    }

    pub fn empty() -> bool{
        true
    }

    pub fn char(c: char) -> Combinator{
        Combinator(Box::new(move |p: & ParserContext| ->bool{
            if p.input_len == p.pos.get(){
                false
            }else if p.input[p.pos.get()] == (c as u8){
                p.pos.set(p.pos.get()+1);
                let prev = p.tree.borrow().tree.clone();
                {
                    p.tree.borrow_mut().tree = Rc::new(TreeLog::Log{prev: prev, val: Tree::Lchar{val: c}});
                }
                true
            }else{
                false
            }
        }))
    }
    
    pub fn anychar() -> Combinator{
        Combinator(Box::new(move |p: & ParserContext| -> bool{
            if p.input_len == p.pos.get(){
                false
            }else{
                {
                    //p.tree.borrow_mut().push(Tree::Lchar{val: p.input[p.pos.get()] as char});
                }
                p.pos.set(p.pos.get()+1);
                true    
            }
        }))
    }
    
    pub fn nonterminal(sym: usize) -> Combinator{
        Combinator(Box::new(move |p: & ParserContext| -> bool{
            let old_p = p.pos.get();
            let old_t = p.tree.borrow().tree.clone();
            let mut initialtree = p.tree.borrow().tree.clone();

            match p.lookup(p.pos.get(),sym) {
                Memos::Fail{ref oldpos} => {
                    if *oldpos != old_p{
                        {
                            //*p.tree.borrow_mut() = Vec::new();
                        }
                        if p.rules[sym](p){
                            //initialtree.push(Tree::Node{sym: sym, child: p.tree.borrow().clone()});
                            p.succ_memo(old_p,sym,p.pos.get(),p.tree.borrow().tree.clone());
                            {
                                //*p.tree.borrow_mut() = initialtree.clone();
                            }
                            true
                        }else{
                            p.fail_memo(old_p,sym);
                            p.pos.set(old_p);
                            {
                                //*p.tree.borrow_mut() = initialtree;
                            }
                            false
                        }
                    }else{
                        true
                    }
                },
                Memos::Memo{ref pos,ref oldpos,child: ref newchild} => {
                    if *oldpos == old_p{
                        println!("bbb");
                        {
                            //p.tree.borrow_mut().push(Tree::Node{sym: sym,child: newchild.clone()});
                        }
                        p.pos.set(*pos);
                        true
                    }else{
                        {
                            //*p.tree.borrow_mut() = Vec::new();
                        }
                        if p.rules[sym](p){
                            //initialtree.push(Tree::Node{sym: sym, child: p.tree.borrow().clone()});
                            p.succ_memo(old_p,sym,p.pos.get(),p.tree.borrow().tree.clone());
                            {
                                //*p.tree.borrow_mut() = initialtree.clone();
                            }
                            true
                        }else{
                            p.fail_memo(old_p,sym);
                            p.pos.set(old_p);
                            {
                                //*p.tree.borrow_mut() = initialtree;
                            }
                            false
                        }
                    }
                
                },
                Memos::Nil => {
                    {
                        *p.tree.borrow_mut() = ContextTree{tree: Rc::new(TreeLog::Nil)};
                    }
                        if p.rules[sym](p){
                            //initialtree.push(Tree::Node{sym: sym, child: p.tree.borrow().clone()});
                            p.succ_memo(old_p,sym,p.pos.get(),p.tree.borrow().tree.clone());
                            {
                                //*p.tree.borrow_mut() = initialtree.clone();
                            }
                            true
                        }else{
                            p.fail_memo(old_p,sym);
                            p.pos.set(old_p);
                            {
                                //*p.tree.borrow_mut() = initialtree;
                            }
                            false
                        }
                        //None => panic!("There is no rule. {}",sym),
                },
            }
        }))
    }
    
    pub fn seq(e1: Combinator,e2: Combinator) -> Combinator{
        Combinator(Box::new(move |p: & ParserContext| -> bool{
            let old_p = p.pos.get();
            let old_t = p.tree.borrow().tree.clone();  
            if e1(p){ 
                if e2(p){
                    true
                }else{
                    p.pos.set(old_p);
                    {
                        p.tree.borrow_mut().set(old_t);
                    }
                    false
                }
            }else{
                p.pos.set(old_p);
                {
                    p.tree.borrow_mut().set(old_t);
                }
                false
            }
        }))
    }
    
    pub fn choice(left: Combinator, right: Combinator) -> Combinator{
        Combinator(Box::new(move |p: & ParserContext| -> bool{
            let old_p = p.pos.get();
            let old_t = p.tree.borrow().tree.clone();
            if left(p){
               true
            }else{
               p.pos.set(old_p);
               {
                    p.tree.borrow_mut().set(old_t.clone());
               }
                if right(p){
                    true
                }else{
                    p.pos.set(old_p);
                    {
                        p.tree.borrow_mut().set(old_t);
                    }
                    false
                }
            }
        }))
    }
    
    pub fn rep(e: Combinator) -> Combinator{
        Combinator(Box::new(move |p: & ParserContext| -> bool{
            loop{
                let old_p = p.pos.get();
                let old_t = p.tree.borrow().tree.clone();
                if !e(p){
                    p.pos.set(old_p);
                    {
                        p.tree.borrow_mut().set(old_t);
                    }
                    break;
                }
            }
            true
        }))
    }
/*
    pub fn rep1(e: Combinator) -> Combinator{
            e - rep0(e)
    }
*/    
    pub fn opt(e: Combinator) -> Combinator{
        Combinator(Box::new(move |p: & ParserContext| -> bool{
            let old_p = p.pos.get();
            let old_t = p.tree.borrow().tree.clone();
            if e(p){
                true
            }else{
                p.pos.set(old_p);
                {
                    p.tree.borrow_mut().set(old_t);
                }
                true
            }
        }))
    }
    
    pub fn not(e: Combinator) -> Combinator{
        Combinator(Box::new(move |p: & ParserContext| -> bool{
            let old_p = p.pos.get();
            let old_t = p.tree.borrow().tree.clone();
           if e(p){
                p.pos.set(old_p);
                {
                    p.tree.borrow_mut().set(old_t);
                }
                false
            }else{
                p.pos.set(old_p);
                {
                    p.tree.borrow_mut().set(old_t);
                }
                true
            }
        }))
    }

    pub fn range(start: char, end: char) -> Combinator{
        Combinator(Box::new(move |p: &ParserContext| -> bool{
            let old_p = p.pos.get();
            if p.input_len == p.pos.get(){
                return false
            }
            let c : u8  = p.input[old_p];
            if start as u8 <= c && c <= end as u8{
                p.pos.set(old_p + 1);
                {
                    //p.tree.borrow_mut().push(Tree::Lchar{val: c as char});
                }
                true
            }else{
                false
            }
        }))
    }
    
    pub fn string(s: &'static str) -> Combinator{
        Combinator(Box::new(move |p: &ParserContext| -> bool{
            let old_p = p.pos.get();
            if old_p + s.len() > p.input_len || old_p >= p.input_len{
                return false
            }
            let ss : Vec<u8> = p.input[(old_p)..=(old_p + s.len()-1)].to_vec();
            if ss == (s.as_bytes()){
                p.pos.set(p.pos.get() + s.len());
                {
                    //p.tree.borrow_mut().push(Tree::Lstr{val: &s});
                }
                true
            }else{
                false
            }
        }))
    }

    pub fn charset(chset: [bool;255])-> Combinator{
        Combinator(Box::new(move |p: & ParserContext| ->bool{
            if p.input_len == p.pos.get(){
                false
            }else if chset[p.input[p.pos.get()] as usize] {
                let c = p.input[p.pos.get()];
                p.pos.set(p.pos.get()+1);
                {
                //   p.tree.borrow_mut().push(Tree::Lchar{val: c as char});
                }
                true
            }else{
                false
            }
        }))
    }
}
