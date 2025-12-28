macro_rules! ast_struct {
    () => {
        # [cfg_attr (not (any (feature = "full" , feature = "derive")) , allow (unknown_lints , unused_macro_rules))] macro_rules ! ast_struct { ($ (# [$ attr : meta]) * $ pub : ident $ struct : ident $ name : ident # full $ body : tt) => { check_keyword_matches ! (pub $ pub) ; check_keyword_matches ! (struct $ struct) ; # [cfg (feature = "full")] $ (# [$ attr]) * $ pub $ struct $ name $ body # [cfg (not (feature = "full"))] $ (# [$ attr]) * $ pub $ struct $ name { _noconstruct : :: std :: marker :: PhantomData <:: proc_macro2 :: Span >, } # [cfg (all (not (feature = "full") , feature = "printing"))] impl :: quote :: ToTokens for $ name { fn to_tokens (& self , _ : & mut :: proc_macro2 :: TokenStream) { unreachable ! () } } } ; ($ (# [$ attr : meta]) * $ pub : ident $ struct : ident $ name : ident $ body : tt) => { check_keyword_matches ! (pub $ pub) ; check_keyword_matches ! (struct $ struct) ; $ (# [$ attr]) * $ pub $ struct $ name $ body } ; }
    };
}

ast_struct!();