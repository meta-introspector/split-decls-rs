// Generated macro for impl_87 (impl)
macro_rules! Depcrate_spanimpl_87 {
() => {
// Module: crate::span
// Provides: {"impl_87"}
// Dependencies: {}
impl cmp :: PartialEq for Span { fn eq (& self , other : & Self) -> bool { match (& self . meta , & other . meta) { (Some (this) , Some (that)) => { this . callsite () == that . callsite () && self . inner == other . inner } _ => false , } } }
};
}
