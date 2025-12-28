macro_rules! deps {
    () => {
        Integer!();
        NInt!();
        NonZero!();
        Unsigned!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < U : Unsigned + NonZero > Integer for NInt < U > { const I8 : i8 = - ((U :: U8 - 1) as i8) - 1 ; const I16 : i16 = - ((U :: U16 - 1) as i16) - 1 ; const I32 : i32 = - ((U :: U32 - 1) as i32) - 1 ; const I64 : i64 = - ((U :: U64 - 1) as i64) - 1 ; # [cfg (feature = "i128")] const I128 : i128 = - ((U :: U128 - 1) as i128) - 1 ; const ISIZE : isize = - ((U :: USIZE - 1) as isize) - 1 ; # [inline] fn to_i8 () -> i8 { Self :: I8 } # [inline] fn to_i16 () -> i16 { Self :: I16 } # [inline] fn to_i32 () -> i32 { Self :: I32 } # [inline] fn to_i64 () -> i64 { Self :: I64 } # [cfg (feature = "i128")] # [inline] fn to_i128 () -> i128 { Self :: I128 } # [inline] fn to_isize () -> isize { Self :: ISIZE } }
    };
}

impl_52!();