macro_rules! deps {
    () => {
        NonZero!();
        PInt!();
        Integer!();
        Unsigned!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < U : Unsigned + NonZero > Integer for PInt < U > { const I8 : i8 = U :: I8 ; const I16 : i16 = U :: I16 ; const I32 : i32 = U :: I32 ; const I64 : i64 = U :: I64 ; # [cfg (feature = "i128")] const I128 : i128 = U :: I128 ; const ISIZE : isize = U :: ISIZE ; # [inline] fn to_i8 () -> i8 { < U as Unsigned > :: to_i8 () } # [inline] fn to_i16 () -> i16 { < U as Unsigned > :: to_i16 () } # [inline] fn to_i32 () -> i32 { < U as Unsigned > :: to_i32 () } # [inline] fn to_i64 () -> i64 { < U as Unsigned > :: to_i64 () } # [cfg (feature = "i128")] # [inline] fn to_i128 () -> i128 { < U as Unsigned > :: to_i128 () } # [inline] fn to_isize () -> isize { < U as Unsigned > :: to_isize () } }
    };
}

impl_51!()