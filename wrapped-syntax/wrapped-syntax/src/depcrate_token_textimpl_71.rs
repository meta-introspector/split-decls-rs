// Generated macro for impl_71 (impl)
macro_rules! Depcrate_token_textimpl_71 {
() => {
// Module: crate::token_text
// Provides: {"impl_71"}
// Dependencies: {}
impl < 'a > TokenText < 'a > { pub fn borrowed (text : & 'a str) -> Self { TokenText (Repr :: Borrowed (text)) } pub (crate) fn owned (green : GreenToken) -> Self { TokenText (Repr :: Owned (green)) } pub fn as_str (& self) -> & str { match & self . 0 { & Repr :: Borrowed (it) => it , Repr :: Owned (green) => green . text () , } } }
};
}
