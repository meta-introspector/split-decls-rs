// Generated macro for Weak (struct)
macro_rules! Depcrate_weakWeak {
() => {
// Module: crate::weak
// Provides: {"Weak"}
// Dependencies: {}
pub (crate) struct Weak < F > { name : & 'static str , addr : AtomicPtr < c_void > , _marker : marker :: PhantomData < F > , }
};
}
