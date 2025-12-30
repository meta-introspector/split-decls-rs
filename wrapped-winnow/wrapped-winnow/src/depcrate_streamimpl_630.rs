// Generated macro for impl_630 (impl)
macro_rules! Depcrate_streamimpl_630 {
() => {
// Module: crate::stream
// Provides: {"impl_630"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'i , T : Clone > Accumulate < & 'i [T] > for VecDeque < T > { # [inline (always)] fn initial (capacity : Option < usize >) -> Self { match capacity { Some (capacity) => VecDeque :: with_capacity (clamp_capacity :: < T > (capacity)) , None => VecDeque :: new () , } } # [inline (always)] fn accumulate (& mut self , acc : & 'i [T]) { self . extend (acc . iter () . cloned ()) ; } }
};
}
