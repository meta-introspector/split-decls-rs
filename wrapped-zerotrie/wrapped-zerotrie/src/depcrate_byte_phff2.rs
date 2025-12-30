// Generated macro for f2 (function)
macro_rules! Depcrate_byte_phff2 {
() => {
// Module: crate::byte_phf
// Provides: {"f2"}
// Dependencies: {}
# [doc = " Calculates the function `f2` for the PHF. For the exact formula, please read the code."] # [doc = ""] # [doc = " When `q == 0`, the operation is a simple modulus."] # [doc = ""] # [doc = " The argument `n` is used only for taking the modulus so that the return value is"] # [doc = " in the range `[0, n)`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use zerotrie::_internal::f2;"] # [doc = " const N: u8 = 10;"] # [doc = ""] # [doc = " // With q = 0:"] # [doc = " assert_eq!(0, f2(0, 0, N));"] # [doc = " assert_eq!(1, f2(1, 0, N));"] # [doc = " assert_eq!(2, f2(2, 0, N));"] # [doc = " assert_eq!(9, f2(9, 0, N));"] # [doc = " assert_eq!(0, f2(10, 0, N));"] # [doc = " assert_eq!(1, f2(11, 0, N));"] # [doc = " assert_eq!(2, f2(12, 0, N));"] # [doc = " assert_eq!(9, f2(19, 0, N));"] # [doc = ""] # [doc = " // With q = 1:"] # [doc = " assert_eq!(1, f2(0, 1, N));"] # [doc = " assert_eq!(0, f2(1, 1, N));"] # [doc = " assert_eq!(3, f2(2, 1, N));"] # [doc = " assert_eq!(8, f2(9, 1, N));"] # [doc = " assert_eq!(1, f2(10, 1, N));"] # [doc = " assert_eq!(0, f2(11, 1, N));"] # [doc = " assert_eq!(3, f2(12, 1, N));"] # [doc = " assert_eq!(8, f2(19, 1, N));"] # [doc = " ```"] # [inline] pub fn f2 (byte : u8 , q : u8 , n : u8) -> u8 { if n == 0 { return byte ; } let mut result = byte ^ q ; for _ in Q_FAST_MAX .. q { result = result ^ (result << 1) ^ (result >> 1) ; } result % n }
};
}
