// Generated macro for impl_509 (impl)
macro_rules! Depcrate_itemimpl_509 {
() => {
// Module: crate::item
// Provides: {"impl_509"}
// Dependencies: {}
impl From < ItemEnum > for DeriveInput { fn from (input : ItemEnum) -> DeriveInput { DeriveInput { attrs : input . attrs , vis : input . vis , ident : input . ident , generics : input . generics , data : Data :: Enum (DataEnum { enum_token : input . enum_token , brace_token : input . brace_token , variants : input . variants , }) , } } }
};
}
