// Generated macro for Zeroizing (struct)
macro_rules! DepcrateZeroizing {
() => {
// Module: crate
// Provides: {"Zeroizing"}
// Dependencies: {}
# [doc = " `Zeroizing` is a wrapper for any `Z: Zeroize` type which implements a"] # [doc = " `Drop` handler which zeroizes dropped values."] # [derive (Debug , Default , Eq , PartialEq)] pub struct Zeroizing < Z : Zeroize > (Z) ;
};
}
