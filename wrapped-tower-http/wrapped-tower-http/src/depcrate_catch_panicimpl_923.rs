// Generated macro for impl_923 (impl)
macro_rules! Depcrate_catch_panicimpl_923 {
() => {
// Module: crate::catch_panic
// Provides: {"impl_923"}
// Dependencies: {}
impl < T , S > Layer < S > for CatchPanicLayer < T > where T : Clone , { type Service = CatchPanic < S , T > ; fn layer (& self , inner : S) -> Self :: Service { CatchPanic { inner , panic_handler : self . panic_handler . clone () , } } }
};
}
