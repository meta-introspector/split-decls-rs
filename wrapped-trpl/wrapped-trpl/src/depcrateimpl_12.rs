// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl Response { # [doc = " Get the full response text."] # [doc = ""] # [doc = " If the response cannot be deserialized, this panics instead of returning"] # [doc = " a [`Result`] (for convenience in the demo)."] pub async fn text (self) -> String { self . 0 . text () . await . unwrap () } }
};
}
