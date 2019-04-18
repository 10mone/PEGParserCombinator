pub mod memo{
    use treelog::treelog::TreeLog;
    use std::rc::Rc;

    #[derive(Debug,Clone)]
    pub enum Memos{
        Fail{oldpos: usize},
        Memo{pos: usize, oldpos: usize, child: Rc<TreeLog>},
        Nil,
    }
}