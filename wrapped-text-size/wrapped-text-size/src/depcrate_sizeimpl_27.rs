// Generated macro for impl_27 (impl)
macro_rules! Depcrate_sizeimpl_27 {
() => {
// Module: crate::size
// Provides: {"impl_27"}
// Dependencies: {}
# [doc = " Methods to act like a primitive integer type, where reasonably applicable."] impl TextSize { # [doc = " Checked addition. Returns `None` if overflow occurred."] # [inline] pub const fn checked_add (self , rhs : TextSize) -> Option < TextSize > { match self . raw . checked_add (rhs . raw) { Some (raw) => Some (TextSize { raw }) , None => None , } } # [doc = " Checked subtraction. Returns `None` if overflow occurred."] # [inline] pub const fn checked_sub (self , rhs : TextSize) -> Option < TextSize > { match self . raw . checked_sub (rhs . raw) { Some (raw) => Some (TextSize { raw }) , None => None , } } }
};
}
