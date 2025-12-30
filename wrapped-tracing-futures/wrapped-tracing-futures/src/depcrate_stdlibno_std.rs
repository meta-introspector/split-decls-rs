// Generated macro for no_std (module)
macro_rules! Depcrate_stdlibno_std {
() => {
// Module: crate::stdlib
// Provides: {"no_std"}
// Dependencies: {}
# [cfg (not (feature = "std"))] mod no_std { # ! [allow (unused_imports)] pub (crate) use core :: { any , array , ascii , cell , char , clone , cmp , convert , default , f32 , f64 , ffi , future , hash , hint , i128 , i16 , i8 , isize , iter , marker , mem , num , ops , option , pin , ptr , result , task , time , u128 , u16 , u32 , u8 , usize , } ; pub (crate) mod borrow { pub (crate) use core :: borrow :: * ; } pub (crate) mod fmt { pub (crate) use core :: fmt :: * ; } pub (crate) mod slice { pub (crate) use core :: slice :: * ; } pub (crate) mod str { pub (crate) use core :: str :: * ; } }
};
}
