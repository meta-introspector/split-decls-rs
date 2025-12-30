// Generated macro for impl_623 (impl)
macro_rules! Depcrate_streamimpl_623 {
() => {
// Module: crate::stream
// Provides: {"impl_623"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'i > Accumulate < & 'i str > for String { # [inline (always)] fn initial (capacity : Option < usize >) -> Self { match capacity { Some (capacity) => String :: with_capacity (clamp_capacity :: < char > (capacity)) , None => String :: new () , } } # [inline (always)] fn accumulate (& mut self , acc : & 'i str) { self . push_str (acc) ; } }
};
}
