macro_rules! current_dir {
    () => {
        # [doc = " Find the directory for your source file"] # [doc (hidden)] # [macro_export] macro_rules ! current_dir { () => { { let root = $ crate :: utils :: cargo_rustc_current_dir ! () ; let file = :: std :: file ! () ; let rel_path = :: std :: path :: Path :: new (file) . parent () . unwrap () ; root . join (rel_path) } } ; }
    };
}

current_dir!();