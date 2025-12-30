// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl < T , F , S > fmt :: Debug for ScopeGuard < T , F , S > where T : fmt :: Debug , F : FnOnce (T) , S : Strategy , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct (stringify ! (ScopeGuard)) . field ("value" , & * self . value) . finish () } }
};
}
