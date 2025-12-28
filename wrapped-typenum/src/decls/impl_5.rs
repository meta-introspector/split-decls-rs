macro_rules! deps {
    () => {
        Bit!();
        B1!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Bit for B1 { const U8 : u8 = 1 ; const BOOL : bool = true ; # [inline] fn new () -> Self { Self } # [inline] fn to_u8 () -> u8 { 1 } # [inline] fn to_bool () -> bool { true } }
    };
}

impl_5!();