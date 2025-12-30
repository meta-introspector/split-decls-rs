// Generated macro for f1 (function)
macro_rules! Depcrate_byte_phff1 {
() => {
// Module: crate::byte_phf
// Provides: {"f1"}
// Dependencies: {}
# [doc = " Calculates the function `f1` for the PHF. For the exact formula, please read the code."] # [doc = ""] # [doc = " When `p == 0`, the operation is a simple modulus."] # [doc = ""] # [doc = " The argument `n` is used only for taking the modulus so that the return value is"] # [doc = " in the range `[0, n)`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use zerotrie::_internal::f1;"] # [doc = " const N: u8 = 10;"] # [doc = ""] # [doc = " // With p = 0:"] # [doc = " assert_eq!(0, f1(0, 0, N));"] # [doc = " assert_eq!(1, f1(1, 0, N));"] # [doc = " assert_eq!(2, f1(2, 0, N));"] # [doc = " assert_eq!(9, f1(9, 0, N));"] # [doc = " assert_eq!(0, f1(10, 0, N));"] # [doc = " assert_eq!(1, f1(11, 0, N));"] # [doc = " assert_eq!(2, f1(12, 0, N));"] # [doc = " assert_eq!(9, f1(19, 0, N));"] # [doc = ""] # [doc = " // With p = 1:"] # [doc = " assert_eq!(1, f1(0, 1, N));"] # [doc = " assert_eq!(0, f1(1, 1, N));"] # [doc = " assert_eq!(2, f1(2, 1, N));"] # [doc = " assert_eq!(2, f1(9, 1, N));"] # [doc = " assert_eq!(4, f1(10, 1, N));"] # [doc = " assert_eq!(5, f1(11, 1, N));"] # [doc = " assert_eq!(1, f1(12, 1, N));"] # [doc = " assert_eq!(7, f1(19, 1, N));"] # [doc = " ```"] # [inline] pub fn f1 (byte : u8 , p : u8 , n : u8) -> u8 { if n == 0 { byte } else if p == 0 { byte % n } else { let result = byte ^ p ^ byte . wrapping_shr (p as u32) ; result % n } }
};
}
