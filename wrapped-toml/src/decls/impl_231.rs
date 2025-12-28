macro_rules! deps {
    () => {
        Index!();
        DeValue!();
        Table!();
    };
}

macro_rules! impl_231 {
    () => {
        deps!();
        impl Index for str { fn index < 'r , 'i > (& self , val : & 'r DeValue < 'i >) -> Option < & 'r Spanned < DeValue < 'i > > > { match * val { DeValue :: Table (ref a) => a . get (self) , _ => None , } } }
    };
}

impl_231!()