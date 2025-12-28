macro_rules! deps {
    () => {
        Piece!();
    };
}

macro_rules! macro_16 {
    () => {
        deps!();
        # [cfg (all (test , target_pointer_width = "64"))] rustc_index :: static_assert_size ! (Piece <'_ >, 16) ;
    };
}

macro_16!();