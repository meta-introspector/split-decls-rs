// Generated macro for create_symbol_identifier (function)
macro_rules! Depcrate_big_endiancreate_symbol_identifier {
() => {
// Module: crate::big_endian
// Provides: {"create_symbol_identifier"}
// Dependencies: {}
# [doc = " Creates an Identifier with name `name` with no wildcards. This, for example,"] # [doc = " can be used to create variables, function names or arbitrary input. Is is"] # [doc = " extremely flexible."] pub fn create_symbol_identifier (arbitrary_string : & str) -> Expression { let identifier_name = create_single_wild_string (arbitrary_string) ; Expression :: Identifier (identifier_name , IdentifierType :: Symbol) }
};
}
