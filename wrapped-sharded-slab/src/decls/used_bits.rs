macro_rules! deps {
    () => {
        Slab!();
        Config!();
    };
}

macro_rules! used_bits {
    () => {
        deps!();
        fn used_bits < C : Config > (key : usize) -> usize { assert_eq ! (C :: RESERVED_BITS + Slab ::< u32 , C >:: USED_BITS , std :: mem :: size_of ::< usize > () * 8) ; key & ((! 0) >> C :: RESERVED_BITS) }
    };
}

used_bits!()