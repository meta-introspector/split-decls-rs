// Generated macro for impl_90 (impl)
macro_rules! Depcrateimpl_90 {
() => {
// Module: crate
// Provides: {"impl_90"}
// Dependencies: {}
impl Span { pub fn cover (self , other : Span) -> Span { if self . anchor != other . anchor { return self ; } let range = self . range . cover (other . range) ; Span { range , .. self } } }
};
}
