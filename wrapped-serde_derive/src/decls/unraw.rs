macro_rules! unraw {
    () => {
        fn unraw (ident : & Ident) -> Ident { Ident :: new (ident . to_string () . trim_start_matches ("r#") , ident . span ()) }
    };
}

unraw!();