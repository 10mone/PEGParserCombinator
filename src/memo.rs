pub mod memo{
    use tree::tree::Tree;

    #[derive(Debug,Clone)]
    pub enum Memos{
        Fail,
        Memo{pos: usize, child: Vec<Tree>},
    }
}