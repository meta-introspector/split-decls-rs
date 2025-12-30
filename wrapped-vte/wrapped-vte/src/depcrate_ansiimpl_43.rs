// Generated macro for impl_43 (impl)
macro_rules! Depcrate_ansiimpl_43 {
() => {
// Module: crate::ansi
// Provides: {"impl_43"}
// Dependencies: {}
impl Sub < Rgb > for Rgb { type Output = Rgb ; fn sub (self , rhs : Rgb) -> Rgb { Rgb { r : self . r . saturating_sub (rhs . r) , g : self . g . saturating_sub (rhs . g) , b : self . b . saturating_sub (rhs . b) , } } }
};
}
