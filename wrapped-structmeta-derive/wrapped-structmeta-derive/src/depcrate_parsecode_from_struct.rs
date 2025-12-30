// Generated macro for code_from_struct (function)
macro_rules! Depcrate_parsecode_from_struct {
() => {
// Module: crate::parse
// Provides: {"code_from_struct"}
// Dependencies: {}
fn code_from_struct (data : & DataStruct) -> Result < TokenStream > { code_from_fields (quote ! (Self) , & data . fields , None) }
};
}
