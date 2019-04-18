pub mod exp{
    use parser_context::parser_context::ParserContext;
    use tree::tree::Tree;
    use memo::memo::Memos;

    #[derive(Debug,Clone)]
    pub enum Exp{
        Empty,
        Char{c:char},
        AnyChar,
        Symbol{sym:&'static str},
        Seq{e1:Box<Exp>, e2:Box<Exp>},
        Choice{e1:Box<Exp>, e2:Box<Exp>},
        Rep{e:Box<Exp>},
        Opt{e:Box<Exp>},
        Not{e:Box<Exp>},
    }

    impl Exp{
        pub fn parse(&self, p: &mut ParserContext) -> bool{ 
        
            match self {
                &Exp::Empty => true,
                &Exp::Char{ref c } => {  //cを使ってマッチするかどうか確かめる
                    if p.input_len == p.pos{    //inputのposで指しているバイト配列の値とcをキャストしたものを比較
                        false
                    }else if p.input[p.pos] == (*c as u8){
                        p.pos += 1;
                        p.tree.push(Tree::Leaf{val: *c});
                        true   
                    }else{
                        false
                    }
                },
                &Exp::AnyChar => {
                    if p.input_len == p.pos{    //空、配列のn番目とpos数が同じならそこは空（配列は0から始まるから）
                        false
                    }else{
                        p.pos += 1;
                        true    
                    }
                }, 
                &Exp::Symbol{ref sym} =>{
                    let old = p.clone();
                    let p_rule = p.clone();
                    let mut initialtree = p.tree.clone();

                    match p.memos.clone().get(&(p.pos,sym)) { 
                        None => {
                            p.tree = Vec::new();
                            match p_rule.rules.get(sym){
                                Some(ref e) => if e.parse(p){
                                    initialtree.push(Tree::Node{sym: sym, child: p.tree.clone()});
                                    p.memos.insert((old.pos, sym), Memos::Memo{pos: p.pos,child: p.tree.clone()});
                                    p.tree = initialtree.clone();
                                //    println!("{:?}",p.tree);
                                    true
                                }else{
                                    p.memos.insert((old.pos, sym), Memos::Fail);
                                    p.pos = old.pos;
                                    p.tree = old.tree;
                                    false
                                },
                                None => panic!("There is no rule. {}",sym),
                            }
                        },
                        Some(ref memo) => {
                            match memo{
                                &Memos::Fail => return false,
                                &Memos::Memo{ref pos,child: ref newchild} => {
                                    p.tree.push(Tree::Node{sym: sym,child: newchild.clone()});
                                    p.pos = *pos;
                                    true
                                }, 
                            }
                        },  
                    }
                },
                &Exp::Seq{ref e1, ref e2} =>{
                    let old = p.clone();   
                    if e1.parse(p){ //parse関数がe1のメソッド呼び
                        if e2.parse(p){
                            true
                        }else{
                            p.pos = old.pos;
                            p.tree = old.tree.clone();
                            false
                        }
                    }else{
                        p.pos = old.pos;
                        p.tree = old.tree;
                        false
                    }
                },  
                &Exp::Choice{ref e1, ref e2} =>{  
                    let old = p.clone();
                    if e1.parse(p){
                        true
                    }else{
                        p.pos = old.pos;
                        p.tree = old.tree.clone();
                        if e2.parse(p){
                            true
                        }else{
                            p.pos = old.pos;
                            p.tree = old.tree;
                            false
                        }
                    }
                },
                &Exp::Rep{ref e} =>{
                    loop{
                        let old = p.clone();
                        if !e.parse(p){
                            p.pos = old.pos;
                            p.tree = old.tree;
                            break;
                        }
                    }
                    true
                },
                &Exp::Opt{ref e} =>{
                    let old = p.clone();
                    if e.parse(p){
                        true
                    }else{
                        p.pos = old.pos;
                        p.tree = old.tree;
                        true
                    }
                },
                &Exp::Not{ref e} =>{
                    let old = p.clone();
                    if e.parse(p){
                        p.pos = old.pos;
                        p.tree = old.tree;
                        false
                    }else{
                        p.pos = old.pos;
                        p.tree = old.tree;
                        true
                    }
                },
                _ => panic!("{:?}: This expression is undefined!", self)
            }   
        }
    }
    pub fn emp() -> Exp{
        Exp::Empty
    }
    pub fn char(c: char) -> Exp{
        Exp::Char{c: c}
    }
    pub fn any() -> Exp{
        Exp::AnyChar
    }
    pub fn sym(sym: &'static str) -> Exp{
        Exp::Symbol{sym: sym}
    }
    pub fn seq(e1: Exp,e2: Exp) -> Exp{
        Exp::Seq{e1: Box::new(e1),e2: Box::new(e2)}
    }
    pub fn choice(e1: Exp,e2: Exp) -> Exp{
        Exp::Choice{e1: Box::new(e1),e2: Box::new(e2)}
    }
    pub fn rep0(e: Exp) -> Exp{
        Exp::Rep{e: Box::new(e)}
    }
    pub fn rep1(e: Exp) -> Exp{
        Exp::Seq{e1: Box::new(e.clone()),e2: Box::new(Exp::Rep{e: Box::new(e)})}
    }
    pub fn opt(e: Exp) -> Exp{
        Exp::Opt{e: Box::new(e)}
    }
    pub fn not(e: Exp) -> Exp{
        Exp::Not{e: Box::new(e)}
    }
}