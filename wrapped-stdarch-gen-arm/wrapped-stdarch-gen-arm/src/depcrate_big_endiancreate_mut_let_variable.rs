// Generated macro for create_mut_let_variable (function)
macro_rules! Depcrate_big_endiancreate_mut_let_variable {
() => {
// Module: crate::big_endian
// Provides: {"create_mut_let_variable"}
// Dependencies: {}
pub fn create_mut_let_variable (variable_name : & str , type_kind : & TypeKind , expression : Expression ,) -> Expression { let identifier_name = create_single_wild_string (variable_name) ; Expression :: Let (LetVariant :: MutWithType (identifier_name , type_kind . clone () , Box :: new (expression) ,)) }
};
}
