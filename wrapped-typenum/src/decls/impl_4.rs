macro_rules! deps {
    () => {
        Bit!();
        B0!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Bit for B0 { const U8 : u8 = 0 ; const BOOL : bool = false ; # [inline] fn new () -> Self { Self } # [inline] fn to_u8 () -> u8 { 0 } # [inline] fn to_bool () -> bool { false } }
    };
}

impl_4!();