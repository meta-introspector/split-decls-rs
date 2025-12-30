// Generated macro for is_test_attribute (function)
macro_rules! Depcrate_proptest_fnis_test_attribute {
() => {
// Module: crate::proptest_fn
// Provides: {"is_test_attribute"}
// Dependencies: {}
fn is_test_attribute (attr : & Attribute) -> bool { let path = match & attr . meta { syn :: Meta :: Path (path) => path , _ => return false , } ; const CANDIDATES_LEN : usize = 4 ; let candidates : [[& str ; CANDIDATES_LEN] ; 2] = [["core" , "prelude" , "*" , "test"] , ["std" , "prelude" , "*" , "test"] ,] ; if path . leading_colon . is_none () && path . segments . len () == 1 && path . segments [0] . arguments . is_none () && path . segments [0] . ident == "test" { return true ; } else if path . segments . len () != candidates [0] . len () { return false ; } candidates . into_iter () . any (| segments | { path . segments . iter () . zip (segments) . all (| (segment , path) | { segment . arguments . is_none () && (path == "*" || segment . ident == path) }) }) }
};
}
