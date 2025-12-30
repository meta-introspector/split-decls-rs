// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl < T , F , S > Drop for ScopeGuard < T , F , S > where F : FnOnce (T) , S : Strategy , { fn drop (& mut self) { let (value , dropfn) = unsafe { (ptr :: read (& * self . value) , ptr :: read (& * self . dropfn)) } ; if S :: should_run () { dropfn (value) ; } } }
};
}
