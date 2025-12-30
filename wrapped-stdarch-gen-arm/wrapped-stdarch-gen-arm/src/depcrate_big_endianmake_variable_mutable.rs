// Generated macro for make_variable_mutable (function)
macro_rules! Depcrate_big_endianmake_variable_mutable {
() => {
// Module: crate::big_endian
// Provides: {"make_variable_mutable"}
// Dependencies: {}
pub fn make_variable_mutable (variable_name : & str , type_kind : & TypeKind) -> Expression { let mut_variable = format ! ("let mut {variable_name}: {type_kind} = {variable_name}") ; let identifier_name = create_single_wild_string (& mut_variable) ; Expression :: Identifier (identifier_name , IdentifierType :: Symbol) }
};
}
