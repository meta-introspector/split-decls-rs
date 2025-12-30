// Generated macro for impl_460 (impl)
macro_rules! Depcrate_stream_statefulimpl_460 {
() => {
// Module: crate::stream::stateful
// Provides: {"impl_460"}
// Dependencies: {}
impl < I , S , U > Compare < U > for Stateful < I , S > where I : Compare < U > , { # [inline (always)] fn compare (& self , other : U) -> CompareResult { self . input . compare (other) } }
};
}
