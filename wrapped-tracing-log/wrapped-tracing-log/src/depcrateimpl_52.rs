// Generated macro for impl_52 (impl)
macro_rules! Depcrateimpl_52 {
() => {
// Module: crate
// Provides: {"impl_52"}
// Dependencies: {}
impl < 'a > AsLog for Metadata < 'a > { type Log = log :: Metadata < 'a > ; fn as_log (& self) -> Self :: Log { log :: Metadata :: builder () . level (self . level () . as_log ()) . target (self . target ()) . build () } }
};
}
