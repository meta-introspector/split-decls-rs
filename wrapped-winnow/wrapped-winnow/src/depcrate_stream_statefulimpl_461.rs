// Generated macro for impl_461 (impl)
macro_rules! Depcrate_stream_statefulimpl_461 {
() => {
// Module: crate::stream::stateful
// Provides: {"impl_461"}
// Dependencies: {}
impl < I , S , T > FindSlice < T > for Stateful < I , S > where I : FindSlice < T > , { # [inline (always)] fn find_slice (& self , substr : T) -> Option < core :: ops :: Range < usize > > { self . input . find_slice (substr) } }
};
}
