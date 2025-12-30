// Generated macro for impl_519 (impl)
macro_rules! Depcrate_itemimpl_519 {
() => {
// Module: crate::item
// Provides: {"impl_519"}
// Dependencies: {}
impl From < DeriveInput > for Item { fn from (input : DeriveInput) -> Item { match input . data { Data :: Struct (data) => Item :: Struct (ItemStruct { attrs : input . attrs , vis : input . vis , struct_token : data . struct_token , ident : input . ident , generics : input . generics , fields : data . fields , semi_token : data . semi_token , }) , Data :: Enum (data) => Item :: Enum (ItemEnum { attrs : input . attrs , vis : input . vis , enum_token : data . enum_token , ident : input . ident , generics : input . generics , brace_token : data . brace_token , variants : data . variants , }) , Data :: Union (data) => Item :: Union (ItemUnion { attrs : input . attrs , vis : input . vis , union_token : data . union_token , ident : input . ident , generics : input . generics , fields : data . fields , }) , } } }
};
}
