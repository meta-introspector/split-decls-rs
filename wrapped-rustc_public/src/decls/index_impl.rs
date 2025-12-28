macro_rules! index_impl {
    () => {
        macro_rules ! index_impl { ($ name : ident) => { impl crate :: IndexedVal for $ name { fn to_val (index : usize) -> Self { $ name (index) } fn to_index (& self) -> usize { self . 0 } } } ; }
    };
}

index_impl!();