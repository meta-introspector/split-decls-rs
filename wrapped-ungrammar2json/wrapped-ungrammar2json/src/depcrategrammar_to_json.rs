// Generated macro for grammar_to_json (function)
macro_rules! Depcrategrammar_to_json {
() => {
// Module: crate
// Provides: {"grammar_to_json"}
// Dependencies: {}
fn grammar_to_json (grammar : & Grammar , mut obj : write_json :: Object < '_ >) { for node in grammar . iter () { let node = & grammar [node] ; rule_to_json (grammar , & node . rule , obj . object (& node . name)) ; } }
};
}
