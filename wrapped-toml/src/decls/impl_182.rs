macro_rules! deps {
    () => {
        DeArray!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl DeArray < '_ > { pub (crate) fn is_array_of_tables (& self) -> bool { self . array_of_tables } pub (crate) fn set_array_of_tables (& mut self , yes : bool) { self . array_of_tables = yes ; } }
    };
}

impl_182!()