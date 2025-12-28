macro_rules! deps {
    () => {
        Array!();
        Index!();
        DeValue!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl Index for usize { fn index < 'r , 'i > (& self , val : & 'r DeValue < 'i >) -> Option < & 'r Spanned < DeValue < 'i > > > { match * val { DeValue :: Array (ref a) => a . get (* self) , _ => None , } } }
    };
}

impl_230!()