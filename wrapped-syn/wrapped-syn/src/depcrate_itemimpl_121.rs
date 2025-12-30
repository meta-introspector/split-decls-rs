// Generated macro for impl_121 (impl)
macro_rules! Depcrate_itemimpl_121 {
() => {
// Module: crate::item
// Provides: {"impl_121"}
// Dependencies: {}
impl From < DeriveInput > for Item { fn from (input : DeriveInput) -> Item { Item { attrs : input . attrs , node : match input . body { Body :: Enum (data) => { ItemEnum { vis : input . vis , enum_token : data . enum_token , ident : input . ident , generics : input . generics , brace_token : data . brace_token , variants : data . variants , } . into () } Body :: Struct (data) => { ItemStruct { vis : input . vis , struct_token : data . struct_token , ident : input . ident , generics : input . generics , data : data . data , semi_token : data . semi_token , } . into () } } , } } }
};
}
