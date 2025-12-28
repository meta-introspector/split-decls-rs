macro_rules! current_rs {
    () => {
        # [doc = " Find the directory for your source file"] # [doc (hidden)] # [macro_export] macro_rules ! current_rs { () => { { let root = $ crate :: utils :: cargo_rustc_current_dir ! () ; let file = :: std :: file ! () ; let rel_path = :: std :: path :: Path :: new (file) ; root . join (rel_path) } } ; }
    };
}

current_rs!()