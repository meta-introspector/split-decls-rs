macro_rules! deps {
    () => {
        Ctxt!();
    };
}

macro_rules! get_multiple_renames {
    () => {
        deps!();
        fn get_multiple_renames (cx : & Ctxt , meta : & ParseNestedMeta ,) -> syn :: Result < (Option < syn :: LitStr > , Vec < syn :: LitStr >) > { let (ser , de) = get_ser_and_de (cx , RENAME , meta , get_lit_str2) ? ; Ok ((ser . at_most_one () , de . get ())) }
    };
}

get_multiple_renames!()