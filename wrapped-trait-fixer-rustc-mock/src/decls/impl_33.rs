macro_rules! deps {
    () => {
        Symbol!();
        MockMetaItem!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl MockMetaItem { pub fn has_name (self , _symbol : Symbol) -> bool { true } }
    };
}

impl_33!()