macro_rules! deps {
    () => {
        CrateDefItems!();
    };
}

macro_rules! impl_crate_def_items {
    () => {
        deps!();
        macro_rules ! impl_crate_def_items { ($ name : ident $ (;) ?) => { impl CrateDefItems for $ name { } } ; }
    };
}

impl_crate_def_items!()