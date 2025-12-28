macro_rules! ast_enum_of_structs {
    () => {
        macro_rules ! ast_enum_of_structs { ($ (# [$ enum_attr : meta]) * $ pub : ident $ enum : ident $ name : ident $ body : tt) => { check_keyword_matches ! (pub $ pub) ; check_keyword_matches ! (enum $ enum) ; $ (# [$ enum_attr]) * $ pub $ enum $ name $ body ast_enum_of_structs_impl ! ($ name $ body) ; # [cfg (feature = "printing")] generate_to_tokens ! (() tokens $ name $ body) ; } ; }
    };
}

ast_enum_of_structs!();