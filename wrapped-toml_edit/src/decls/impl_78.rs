macro_rules! deps {
    () => {
        Item!();
        Index!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl Index for String { fn index < 'v > (& self , v : & 'v Item) -> Option < & 'v Item > { self [..] . index (v) } fn index_mut < 'v > (& self , v : & 'v mut Item) -> Option < & 'v mut Item > { self [..] . index_mut (v) } }
    };
}

impl_78!();