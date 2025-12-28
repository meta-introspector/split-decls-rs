macro_rules! deps {
    () => {
        Container!();
    };
}

macro_rules! pretend_used {
    () => {
        deps!();
        pub fn pretend_used (cont : & Container , is_packed : bool) -> TokenStream { let pretend_fields = pretend_fields_used (cont , is_packed) ; let pretend_variants = pretend_variants_used (cont) ; quote ! { # pretend_fields # pretend_variants } }
    };
}

pretend_used!();