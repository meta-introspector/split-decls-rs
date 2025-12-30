// Generated macro for remove_item (function)
macro_rules! Depcrateremove_item {
() => {
// Module: crate
// Provides: {"remove_item"}
// Dependencies: {}
fn remove_item (cont : & mut Vec < String > , item : & String) -> Option < String > { match cont . iter () . position (| x | * x == * item) { Some (pos) => Some (cont . remove (pos)) , None => None , } }
};
}
