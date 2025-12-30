// Generated macro for with_contract_check (function)
macro_rules! Depcrate_astwith_contract_check {
() => {
// Module: crate::ast
// Provides: {"with_contract_check"}
// Dependencies: {}
fn with_contract_check (skip_deserializing : bool , skip_serializing : bool , action : TokenStream ,) -> TokenStream { match (skip_deserializing , skip_serializing) { (true , true) => TokenStream :: new () , (true , false) => quote ! { if # GENERATOR . contract () . is_serialize () { # action } } , (false , true) => quote ! { if # GENERATOR . contract () . is_deserialize () { # action } } , (false , false) => action , } }
};
}
