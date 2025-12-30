// Generated macro for is_valid_ident (function)
macro_rules! Depcrate_identifieris_valid_ident {
() => {
// Module: crate::identifier
// Provides: {"is_valid_ident"}
// Dependencies: {}
# [doc = " Returns whether a string is a valid JavaScript identifier."] # [doc = " Defined at https://tc39.es/ecma262/#prod-IdentifierName."] pub fn is_valid_ident (name : & str) -> bool { maybe_valid_chars (name) . all (| opt | opt . is_some ()) }
};
}
