// Generated macro for impl_93 (impl)
macro_rules! Depcrate_subscriberimpl_93 {
() => {
// Module: crate::subscriber
// Provides: {"impl_93"}
// Dependencies: {}
impl < F > Running < F > where F : Fn (& Metadata < '_ >) -> bool , { fn lookup_current (& self) -> Option < span :: Id > { let stack = self . current . lock () . unwrap () ; stack . last () . cloned () } }
};
}
