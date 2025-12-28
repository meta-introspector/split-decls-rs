macro_rules! doc_comment {
    () => {
        macro_rules ! doc_comment { ($ x : expr , $ ($ tt : tt) *) => { # [doc = $ x] $ ($ tt) * } ; }
    };
}

doc_comment!();