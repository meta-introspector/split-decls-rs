macro_rules! other_1 {
    () => {
        # [cfg (feature = "in-rust-tree")] extern crate rustc_lexer ;
    };
}

other_1!()