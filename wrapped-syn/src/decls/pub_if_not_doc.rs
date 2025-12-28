macro_rules! pub_if_not_doc {
    () => {
        # [cfg (all (not (doc) , feature = "parsing"))] macro_rules ! pub_if_not_doc { ($ (# [$ m : meta]) * $ pub : ident $ ($ item : tt) *) => { check_keyword_matches ! (pub $ pub) ; $ (# [$ m]) * $ pub $ ($ item) * } ; }
    };
}

pub_if_not_doc!();