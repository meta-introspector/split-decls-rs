// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl < T , F , S > DerefMut for ScopeGuard < T , F , S > where F : FnOnce (T) , S : Strategy , { fn deref_mut (& mut self) -> & mut T { & mut * self . value } }
};
}
