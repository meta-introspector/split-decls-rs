// Generated macro for impl_603 (impl)
macro_rules! Depcrate_litimpl_603 {
() => {
// Module: crate::lit
// Provides: {"impl_603"}
// Dependencies: {}
impl From < Literal > for LitInt { # [track_caller] fn from (token : Literal) -> Self { let repr = token . to_string () ; if let Some ((digits , suffix)) = value :: parse_lit_int (& repr) { LitInt { repr : Box :: new (LitIntRepr { token , digits , suffix , }) , } } else { panic ! ("not an integer literal: `{}`" , repr) ; } } }
};
}
