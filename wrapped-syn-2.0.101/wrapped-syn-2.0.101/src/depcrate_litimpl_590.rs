// Generated macro for impl_590 (impl)
macro_rules! Depcrate_litimpl_590 {
() => {
// Module: crate::lit
// Provides: {"impl_590"}
// Dependencies: {}
impl From < Literal > for LitInt { fn from (token : Literal) -> Self { let repr = token . to_string () ; if let Some ((digits , suffix)) = value :: parse_lit_int (& repr) { LitInt { repr : Box :: new (LitIntRepr { token , digits , suffix , }) , } } else { panic ! ("not an integer literal: `{}`" , repr) ; } } }
};
}
