// Generated macro for rule_to_json (function)
macro_rules! Depcraterule_to_json {
() => {
// Module: crate
// Provides: {"rule_to_json"}
// Dependencies: {}
fn rule_to_json (grammar : & Grammar , rule : & Rule , mut obj : write_json :: Object < '_ >) { match rule { Rule :: Labeled { label , rule } => { obj . string ("label" , label) ; rule_to_json (grammar , rule , obj . object ("rule")) } Rule :: Node (node) => { obj . string ("node" , & grammar [* node] . name) ; } Rule :: Token (token) => { obj . string ("token" , & grammar [* token] . name) ; } Rule :: Seq (rules) | Rule :: Alt (rules) => { let tag = match rule { Rule :: Seq (_) => "seq" , Rule :: Alt (_) => "alt" , _ => unreachable ! () , } ; let mut array = obj . array (tag) ; for rule in rules { rule_to_json (grammar , rule , array . object ()) ; } } Rule :: Opt (arg) | Rule :: Rep (arg) => { let tag = match rule { Rule :: Opt (_) => "opt" , Rule :: Rep (_) => "rep" , _ => unreachable ! () , } ; rule_to_json (grammar , arg , obj . object (tag)) ; } } }
};
}
