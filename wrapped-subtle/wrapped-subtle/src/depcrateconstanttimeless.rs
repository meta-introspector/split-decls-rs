// Generated macro for ConstantTimeLess (trait)
macro_rules! DepcrateConstantTimeLess {
() => {
// Module: crate
// Provides: {"ConstantTimeLess"}
// Dependencies: {}
# [doc = " A type which can be compared in some manner and be determined to be less"] # [doc = " than another of the same type."] pub trait ConstantTimeLess : ConstantTimeEq + ConstantTimeGreater { # [doc = " Determine whether `self < other`."] # [doc = ""] # [doc = " The bitwise-NOT of the return value of this function should be usable to"] # [doc = " determine if `self >= other`."] # [doc = ""] # [doc = " A default implementation is provided and implemented for the unsigned"] # [doc = " integer types."] # [doc = ""] # [doc = " This function should execute in constant time."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " A `Choice` with a set bit if `self < other`, and with no set bits"] # [doc = " otherwise."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use subtle::ConstantTimeLess;"] # [doc = ""] # [doc = " let x: u8 = 13;"] # [doc = " let y: u8 = 42;"] # [doc = ""] # [doc = " let x_lt_y = x.ct_lt(&y);"] # [doc = ""] # [doc = " assert_eq!(x_lt_y.unwrap_u8(), 1);"] # [doc = ""] # [doc = " let y_lt_x = y.ct_lt(&x);"] # [doc = ""] # [doc = " assert_eq!(y_lt_x.unwrap_u8(), 0);"] # [doc = ""] # [doc = " let x_lt_x = x.ct_lt(&x);"] # [doc = ""] # [doc = " assert_eq!(x_lt_x.unwrap_u8(), 0);"] # [doc = " ```"] # [inline] fn ct_lt (& self , other : & Self) -> Choice { ! self . ct_gt (other) & ! self . ct_eq (other) } }
};
}
