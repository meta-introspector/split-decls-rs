// Generated macro for impl_235 (impl)
macro_rules! Depcrate_parser_astimpl_235 {
() => {
// Module: crate::parser::ast
// Provides: {"impl_235"}
// Dependencies: {}
impl StringConcat { pub (crate) fn to_template_string (& self) -> String { let mut res = Vec :: new () ; for value in & self . values { match value { ExprVal :: String (ref s) => res . push (format ! ("'{}'" , s)) , ExprVal :: Ident (ref s) => res . push (s . to_string ()) , _ => res . push ("unknown" . to_string ()) , } } res . join (" ~ ") } }
};
}
