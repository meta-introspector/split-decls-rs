// Generated macro for impl_42 (impl)
macro_rules! Depcrate_ansiimpl_42 {
() => {
// Module: crate::ansi
// Provides: {"impl_42"}
// Dependencies: {}
impl Add < Rgb > for Rgb { type Output = Rgb ; fn add (self , rhs : Rgb) -> Rgb { Rgb { r : self . r . saturating_add (rhs . r) , g : self . g . saturating_add (rhs . g) , b : self . b . saturating_add (rhs . b) , } } }
};
}
