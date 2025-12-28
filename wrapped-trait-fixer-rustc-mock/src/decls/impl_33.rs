macro_rules! deps {
    () => {
        MockMetaItem!();
        Symbol!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl MockMetaItem { pub fn has_name (self , _symbol : Symbol) -> bool { true } }
    };
}

impl_33!();