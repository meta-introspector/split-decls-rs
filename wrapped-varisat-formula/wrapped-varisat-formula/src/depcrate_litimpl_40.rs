// Generated macro for impl_40 (impl)
macro_rules! Depcrate_litimpl_40 {
() => {
// Module: crate::lit
// Provides: {"impl_40"}
// Dependencies: {}
impl ops :: BitXor < bool > for Lit { type Output = Lit ; # [inline] fn bitxor (self , rhs : bool) -> Lit { Lit { code : self . code ^ (rhs as LitIdx) , } } }
};
}
