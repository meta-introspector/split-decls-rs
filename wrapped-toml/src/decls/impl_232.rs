macro_rules! deps {
    () => {
        Index!();
        DeValue!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        impl Index for String { fn index < 'r , 'i > (& self , val : & 'r DeValue < 'i >) -> Option < & 'r Spanned < DeValue < 'i > > > { self [..] . index (val) } }
    };
}

impl_232!()