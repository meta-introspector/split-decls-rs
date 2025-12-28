macro_rules! IndexedVal {
    () => {
        pub trait IndexedVal { fn to_val (index : usize) -> Self ; fn to_index (& self) -> usize ; }
    };
}

IndexedVal!();