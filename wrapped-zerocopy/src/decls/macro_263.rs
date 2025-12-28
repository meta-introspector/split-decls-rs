macro_rules! macro_263 {
    () => {
        assert_unaligned ! (ManuallyDrop < () >, ManuallyDrop < u8 >) ;
    };
}

macro_263!();