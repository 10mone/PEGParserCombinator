pub mod treelog{
    use tree::tree::Tree;
    use std::rc::Rc;

    #[derive(Debug,Clone)]
    pub enum TreeLog{
        Nil,
        Log{prev: Rc<TreeLog>, val: Tree},
    }

    impl TreeLog{
        pub fn log2tree(&self) -> Rc<Vec<Tree>>{
            let mut vtree = Vec::new();
            self.f(&mut vtree);
            Rc::new(vtree)
        }

        fn f(&self, vtree: &mut Vec<Tree>){
            match self{
                TreeLog::Nil =>(),
                TreeLog::Log{ref prev,ref val} =>{
                    prev.f(vtree);
                    vtree.push(val.clone());
                }
            }
        }

        pub fn push(prev: Rc<TreeLog>,val: Tree) -> TreeLog{
            TreeLog::Log{prev: prev, val: val}
        }
        
    }
}