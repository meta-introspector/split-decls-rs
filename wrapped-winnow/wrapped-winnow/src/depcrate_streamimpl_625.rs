// Generated macro for impl_625 (impl)
macro_rules! Depcrate_streamimpl_625 {
() => {
// Module: crate::stream
// Provides: {"impl_625"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Accumulate < String > for String { # [inline (always)] fn initial (capacity : Option < usize >) -> Self { match capacity { Some (capacity) => String :: with_capacity (clamp_capacity :: < char > (capacity)) , None => String :: new () , } } # [inline (always)] fn accumulate (& mut self , acc : String) { self . push_str (& acc) ; } }
};
}
