macro_rules! deps {
    () => {
        Symbol!();
        Ctxt!();
    };
}

macro_rules! get_lit_str {
    () => {
        deps!();
        fn get_lit_str (cx : & Ctxt , attr_name : Symbol , meta : & ParseNestedMeta ,) -> syn :: Result < Option < syn :: LitStr > > { get_lit_str2 (cx , attr_name , attr_name , meta) }
    };
}

get_lit_str!();