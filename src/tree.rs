pub mod tree{
    use std::rc::Rc;
    #[derive(Debug,Clone)]
    pub enum Tree{
        //Node{sym: &'static str, child: Vec<Tree>},
        Node{sym: usize, child: Rc<Vec<Tree>>},
        Lchar{val: char},
        Lstr{val: &'static str}
    }

    impl Tree{
        pub fn two_string(&self, symbol: &[&'static str]) -> String{
            match self{
                &Tree::Lchar{ref val} => format!("{}",val),
                &Tree::Lstr{ref val} => format!("{}",val),
                &Tree::Node{ref sym, ref child} => format!("[{}{}]",symbol[*sym],{
                    //アキュムレーターにくっつけた結果をパシパシ
                    child.iter().fold("".to_string(), |acc,child| format!("{} {}",acc,child.two_string(symbol)))
                }),
            }
        }
    }
}