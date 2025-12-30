// Generated macro for impl_522 (impl)
macro_rules! Depcrate_itemimpl_522 {
() => {
// Module: crate::item
// Provides: {"impl_522"}
// Dependencies: {}
impl From < ItemUnion > for DeriveInput { fn from (input : ItemUnion) -> DeriveInput { DeriveInput { attrs : input . attrs , vis : input . vis , ident : input . ident , generics : input . generics , data : Data :: Union (DataUnion { union_token : input . union_token , fields : input . fields , }) , } } }
};
}
