// Generated macro for impl_593 (impl)
macro_rules! Depcrate_litimpl_593 {
() => {
// Module: crate::lit
// Provides: {"impl_593"}
// Dependencies: {}
impl From < Literal > for LitFloat { fn from (token : Literal) -> Self { let repr = token . to_string () ; if let Some ((digits , suffix)) = value :: parse_lit_float (& repr) { LitFloat { repr : Box :: new (LitFloatRepr { token , digits , suffix , }) , } } else { panic ! ("not a float literal: `{}`" , repr) ; } } }
};
}
