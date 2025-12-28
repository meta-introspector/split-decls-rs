macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! Index {
    () => {
        deps!();
        pub trait Index : crate :: private :: Sealed { # [doc (hidden)] fn index < 'v > (& self , val : & 'v Item) -> Option < & 'v Item > ; # [doc (hidden)] fn index_mut < 'v > (& self , val : & 'v mut Item) -> Option < & 'v mut Item > ; }
    };
}

Index!()