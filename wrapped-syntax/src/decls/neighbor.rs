macro_rules! deps {
    () => {
        AstNode!();
    };
}

macro_rules! neighbor {
    () => {
        deps!();
        pub fn neighbor < T : AstNode > (me : & T , direction : Direction) -> Option < T > { me . syntax () . siblings (direction) . skip (1) . find_map (T :: cast) }
    };
}

neighbor!()