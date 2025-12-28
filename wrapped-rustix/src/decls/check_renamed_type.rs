macro_rules! check_renamed_type {
    () => {
        # [doc = " The same as `check_type`, but for unions and anonymous structs we've"] # [doc = " renamed to avoid having types like `bindgen_ty_1` in the API."] macro_rules ! check_renamed_type { ($ to : ident , $ from : ident) => { assert_eq_size ! ($ to , c ::$ from) ; assert_eq_align ! ($ to , c ::$ from) ; } ; }
    };
}

check_renamed_type!();