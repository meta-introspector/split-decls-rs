macro_rules! FLOAT_POW5_INV_BITCOUNT {
    () => {
        pub const FLOAT_POW5_INV_BITCOUNT : i32 = d2s :: DOUBLE_POW5_INV_BITCOUNT - 64 ;
    };
}

FLOAT_POW5_INV_BITCOUNT!()