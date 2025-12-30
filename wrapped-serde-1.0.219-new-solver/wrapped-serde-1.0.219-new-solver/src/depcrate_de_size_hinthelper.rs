// Generated macro for helper (function)
macro_rules! Depcrate_de_size_hinthelper {
() => {
// Module: crate::de::size_hint
// Provides: {"helper"}
// Dependencies: {}
fn helper (bounds : (usize , Option < usize >)) -> Option < usize > { match bounds { (lower , Some (upper)) if lower == upper => Some (upper) , _ => None , } }
};
}
