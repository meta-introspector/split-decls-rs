macro_rules! mut_if {
    () => {
        fn mut_if (is_mut : bool) -> Option < TokenStream > { if is_mut { Some (quote ! (mut)) } else { None } }
    };
}

mut_if!()