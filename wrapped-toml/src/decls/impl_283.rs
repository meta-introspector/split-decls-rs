macro_rules! deps {
    () => {
        Table!();
    };
}

macro_rules! impl_283 {
    () => {
        deps!();
        impl Table { pub (crate) fn body_mut (& mut self) -> & mut String { & mut self . body } pub (crate) fn has_children (& mut self , yes : bool) { self . has_children = yes ; } }
    };
}

impl_283!()