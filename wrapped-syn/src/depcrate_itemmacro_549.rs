// Generated macro for macro_549 (macro)
macro_rules! Depcrate_itemmacro_549 {
() => {
// Module: crate::item
// Provides: {"macro_549"}
// Dependencies: {}
ast_struct ! { # [doc = " The variadic argument of a foreign function."] # [doc = ""] # [doc = " ```rust"] # [doc = " # struct c_char;"] # [doc = " # struct c_int;"] # [doc = " #"] # [doc = " extern \"C\" {"] # [doc = "     fn printf(format: *const c_char, ...) -> c_int;"] # [doc = "     //                               ^^^"] # [doc = " }"] # [doc = " ```"] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct Variadic { pub attrs : Vec < Attribute >, pub pat : Option < (Box < Pat >, Token ! [:]) >, pub dots : Token ! [...] , pub comma : Option < Token ! [,] >, } }
};
}
