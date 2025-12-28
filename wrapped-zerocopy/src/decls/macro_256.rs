macro_rules! macro_256 {
    () => {
        assert_unaligned ! (CoreMaybeUninit < () >, CoreMaybeUninit < u8 >) ;
    };
}

macro_256!()