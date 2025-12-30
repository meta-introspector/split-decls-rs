// Generated macro for validate_block_structure (function)
macro_rules! Depcrate_validationvalidate_block_structure {
() => {
// Module: crate::validation
// Provides: {"validate_block_structure"}
// Dependencies: {}
pub (crate) fn validate_block_structure (root : & SyntaxNode) { let mut stack = Vec :: new () ; for node in root . descendants_with_tokens () { match node . kind () { T ! ['{'] => stack . push (node) , T ! ['}'] => { if let Some (pair) = stack . pop () { assert_eq ! (node . parent () , pair . parent () , "\nunpaired curlies:\n{}\n{:#?}\n" , root . text () , root ,) ; assert ! (node . next_sibling_or_token () . is_none () && pair . prev_sibling_or_token () . is_none () , "\nfloating curlies at {:?}\nfile:\n{}\nerror:\n{}\n" , node , root . text () , node ,) ; } } _ => () , } } }
};
}
