// Generated macro for impl_89 (impl)
macro_rules! Depcrateimpl_89 {
() => {
// Module: crate
// Provides: {"impl_89"}
// Dependencies: {}
impl Span { pub fn cover (self , other : Span) -> Span { if self . anchor != other . anchor { return self ; } let range = self . range . cover (other . range) ; Span { range , .. self } } }
};
}
