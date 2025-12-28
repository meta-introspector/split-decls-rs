macro_rules! provide {
    () => {
        pub fn provide (providers : & mut Providers) { * providers = Providers { symbol_name : symbol_name_provider , .. * providers } ; }
    };
}

provide!();