// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl < T , F , S > Deref for ScopeGuard < T , F , S > where F : FnOnce (T) , S : Strategy , { type Target = T ; fn deref (& self) -> & T { & * self . value } }
};
}
