// Generated macro for impl_174 (impl)
macro_rules! Depcrateimpl_174 {
() => {
// Module: crate
// Provides: {"impl_174"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a str > for Url { type Error = ParseError ; fn try_from (s : & 'a str) -> Result < Self , Self :: Error > { Self :: parse (s) } }
};
}
