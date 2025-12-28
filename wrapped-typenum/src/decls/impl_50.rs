macro_rules! deps {
    () => {
        Z0!();
        Integer!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl Integer for Z0 { const I8 : i8 = 0 ; const I16 : i16 = 0 ; const I32 : i32 = 0 ; const I64 : i64 = 0 ; # [cfg (feature = "i128")] const I128 : i128 = 0 ; const ISIZE : isize = 0 ; # [inline] fn to_i8 () -> i8 { 0 } # [inline] fn to_i16 () -> i16 { 0 } # [inline] fn to_i32 () -> i32 { 0 } # [inline] fn to_i64 () -> i64 { 0 } # [cfg (feature = "i128")] # [inline] fn to_i128 () -> i128 { 0 } # [inline] fn to_isize () -> isize { 0 } }
    };
}

impl_50!()