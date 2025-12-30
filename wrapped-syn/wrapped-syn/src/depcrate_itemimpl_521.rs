// Generated macro for impl_521 (impl)
macro_rules! Depcrate_itemimpl_521 {
() => {
// Module: crate::item
// Provides: {"impl_521"}
// Dependencies: {}
impl From < ItemEnum > for DeriveInput { fn from (input : ItemEnum) -> DeriveInput { DeriveInput { attrs : input . attrs , vis : input . vis , ident : input . ident , generics : input . generics , data : Data :: Enum (DataEnum { enum_token : input . enum_token , brace_token : input . brace_token , variants : input . variants , }) , } } }
};
}
