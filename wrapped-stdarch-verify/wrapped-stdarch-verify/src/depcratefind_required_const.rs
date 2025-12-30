// Generated macro for find_required_const (function)
macro_rules! Depcratefind_required_const {
() => {
// Module: crate
// Provides: {"find_required_const"}
// Dependencies: {}
fn find_required_const (name : & str , attrs : & [syn :: Attribute]) -> Vec < usize > { attrs . iter () . filter_map (| a | { if let syn :: Meta :: List (ref l) = a . meta { Some (l) } else { None } }) . flat_map (| l | { if l . path . segments [0] . ident == name { syn :: parse2 :: < RustcArgsRequiredConst > (l . tokens . clone ()) . unwrap () . args } else { Vec :: new () } }) . collect () }
};
}
