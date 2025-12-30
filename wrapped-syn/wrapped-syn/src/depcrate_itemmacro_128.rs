// Generated macro for macro_128 (macro)
macro_rules! Depcrate_itemmacro_128 {
() => {
// Module: crate::item
// Provides: {"macro_128"}
// Dependencies: {}
ast_struct ! { # [doc = " Represents an item declaration within a trait declaration,"] # [doc = " possibly including a default implementation. A trait item is"] # [doc = " either required (meaning it doesn't have an implementation, just a"] # [doc = " signature) or provided (meaning it has a default implementation)."] pub struct TraitItem { pub attrs : Vec < Attribute >, pub node : TraitItemKind , } }
};
}
