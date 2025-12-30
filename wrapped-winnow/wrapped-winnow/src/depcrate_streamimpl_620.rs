// Generated macro for impl_620 (impl)
macro_rules! Depcrate_streamimpl_620 {
() => {
// Module: crate::stream
// Provides: {"impl_620"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T > Accumulate < T > for Vec < T > { # [inline (always)] fn initial (capacity : Option < usize >) -> Self { match capacity { Some (capacity) => Vec :: with_capacity (clamp_capacity :: < T > (capacity)) , None => Vec :: new () , } } # [inline (always)] fn accumulate (& mut self , acc : T) { self . push (acc) ; } }
};
}
