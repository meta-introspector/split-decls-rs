// Generated macro for create_let_variable (function)
macro_rules! Depcrate_big_endiancreate_let_variable {
() => {
// Module: crate::big_endian
// Provides: {"create_let_variable"}
// Dependencies: {}
# [doc = " Creates: `let <variable_name>: <type> = <expression>`"] pub fn create_let_variable (variable_name : & str , type_kind : & TypeKind , expression : Expression ,) -> Expression { let identifier_name = create_single_wild_string (variable_name) ; Expression :: Let (LetVariant :: WithType (identifier_name , type_kind . clone () , Box :: new (expression) ,)) }
};
}
