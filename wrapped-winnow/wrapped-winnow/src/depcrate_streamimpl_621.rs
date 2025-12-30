// Generated macro for impl_621 (impl)
macro_rules! Depcrate_streamimpl_621 {
() => {
// Module: crate::stream
// Provides: {"impl_621"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'i , T : Clone > Accumulate < & 'i [T] > for Vec < T > { # [inline (always)] fn initial (capacity : Option < usize >) -> Self { match capacity { Some (capacity) => Vec :: with_capacity (clamp_capacity :: < T > (capacity)) , None => Vec :: new () , } } # [inline (always)] fn accumulate (& mut self , acc : & 'i [T]) { self . extend (acc . iter () . cloned ()) ; } }
};
}
