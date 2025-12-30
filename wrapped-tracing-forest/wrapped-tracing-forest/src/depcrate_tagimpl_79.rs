// Generated macro for impl_79 (impl)
macro_rules! Depcrate_tagimpl_79 {
() => {
// Module: crate::tag
// Provides: {"impl_79"}
// Dependencies: {}
impl < F > TagParser for F where F : 'static + Fn (& Event) -> Option < Tag > , { fn parse (& self , event : & Event) -> Option < Tag > { self (event) } }
};
}
