// Generated macro for impl_606 (impl)
macro_rules! Depcrate_litimpl_606 {
() => {
// Module: crate::lit
// Provides: {"impl_606"}
// Dependencies: {}
impl From < Literal > for LitFloat { # [track_caller] fn from (token : Literal) -> Self { let repr = token . to_string () ; if let Some ((digits , suffix)) = value :: parse_lit_float (& repr) { LitFloat { repr : Box :: new (LitFloatRepr { token , digits , suffix , }) , } } else { panic ! ("not a float literal: `{}`" , repr) ; } } }
};
}
