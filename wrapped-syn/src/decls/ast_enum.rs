macro_rules! ast_enum {
    () => {
        # [cfg (any (feature = "full" , feature = "derive"))] macro_rules ! ast_enum { ($ (# [$ enum_attr : meta]) * $ pub : ident $ enum : ident $ name : ident $ body : tt) => { check_keyword_matches ! (pub $ pub) ; check_keyword_matches ! (enum $ enum) ; $ (# [$ enum_attr]) * $ pub $ enum $ name $ body } ; }
    };
}

ast_enum!();