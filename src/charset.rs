pub mod charset{
    use std::ops::Sub;
    use combinator::combinator::charset;

    #[derive(Debug)]
    pub enum CharSet{
        Char(u8),
        Range(u8, u8),
    }

    impl Sub for CharSet {
        type Output = CharSet;

        fn sub(self, other: CharSet) -> CharSet {
            match self{
                CharSet::Char(c1) => match other{
                    CharSet::Char(c2) => CharSet::Range(c1,c2),
                    CharSet::Range(_, c2) => CharSet::Range(c1,c2),
                },
                CharSet::Range(c1, _) => match other {
                    CharSet::Char(c2) => CharSet::Range(c1, c2),
                    CharSet::Range(_, c2) => CharSet::Range(c1, c2),
                }
            }
        }
    }

    #[macro_export]
    macro_rules! charset {
        ( $( $x:expr ),* ) => {
            {
                let mut chset = [false;255];
                $(
                    match $x{
                        CharSet::Char(c) => chset[c as usize] = true,
                        CharSet::Range(c1,c2) => {
                            for c in c1..=c2{
                                chset[c as usize] = true;
                            }
                        }
                    }
                )*
                charset(chset)
            }
        };
    }

    pub fn ch(c: char) -> CharSet{
        CharSet::Char(c as u8)
    }
}