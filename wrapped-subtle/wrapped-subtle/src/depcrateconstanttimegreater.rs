// Generated macro for ConstantTimeGreater (trait)
macro_rules! DepcrateConstantTimeGreater {
() => {
// Module: crate
// Provides: {"ConstantTimeGreater"}
// Dependencies: {}
# [doc = " A type which can be compared in some manner and be determined to be greater"] # [doc = " than another of the same type."] pub trait ConstantTimeGreater { # [doc = " Determine whether `self > other`."] # [doc = ""] # [doc = " The bitwise-NOT of the return value of this function should be usable to"] # [doc = " determine if `self <= other`."] # [doc = ""] # [doc = " This function should execute in constant time."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " A `Choice` with a set bit if `self > other`, and with no set bits"] # [doc = " otherwise."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use subtle::ConstantTimeGreater;"] # [doc = ""] # [doc = " let x: u8 = 13;"] # [doc = " let y: u8 = 42;"] # [doc = ""] # [doc = " let x_gt_y = x.ct_gt(&y);"] # [doc = ""] # [doc = " assert_eq!(x_gt_y.unwrap_u8(), 0);"] # [doc = ""] # [doc = " let y_gt_x = y.ct_gt(&x);"] # [doc = ""] # [doc = " assert_eq!(y_gt_x.unwrap_u8(), 1);"] # [doc = ""] # [doc = " let x_gt_x = x.ct_gt(&x);"] # [doc = ""] # [doc = " assert_eq!(x_gt_x.unwrap_u8(), 0);"] # [doc = " ```"] fn ct_gt (& self , other : & Self) -> Choice ; }
};
}
