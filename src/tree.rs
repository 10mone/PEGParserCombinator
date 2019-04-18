pub mod tree{
    #[derive(Debug,Clone)]
    pub enum Tree{
        Node{sym: &'static str, child: Vec<Tree>},
        Leaf{val: char}
    }

    impl Tree{
        pub fn two_string(&self) -> String{
            match self{
                &Tree::Leaf{ref val} => format!("{}",val),
                &Tree::Node{ref sym, ref child} => format!("[{}{}]",sym,{
                    //アキュムレーターにくっつけた結果をパシパシ
                    child.iter().fold("".to_string(), |acc,child| format!("{} {}",acc,child.two_string()))
                }),
            }
        }
    }

}