// Generated macro for impl_622 (impl)
macro_rules! Depcrate_streamimpl_622 {
() => {
// Module: crate::stream
// Provides: {"impl_622"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Accumulate < char > for String { # [inline (always)] fn initial (capacity : Option < usize >) -> Self { match capacity { Some (capacity) => String :: with_capacity (clamp_capacity :: < char > (capacity)) , None => String :: new () , } } # [inline (always)] fn accumulate (& mut self , acc : char) { self . push (acc) ; } }
};
}
