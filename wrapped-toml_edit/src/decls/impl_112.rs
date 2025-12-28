macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl Item { # [doc = " Sets `self` to the given item if `self` is none and"] # [doc = " returns a mutable reference to `self`."] pub fn or_insert (& mut self , item : Self) -> & mut Self { if self . is_none () { * self = item ; } self } }
    };
}

impl_112!();