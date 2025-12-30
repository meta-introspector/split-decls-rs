// Generated macro for impl_624 (impl)
macro_rules! Depcrate_streamimpl_624 {
() => {
// Module: crate::stream
// Provides: {"impl_624"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'i > Accumulate < Cow < 'i , str > > for String { # [inline (always)] fn initial (capacity : Option < usize >) -> Self { match capacity { Some (capacity) => String :: with_capacity (clamp_capacity :: < char > (capacity)) , None => String :: new () , } } # [inline (always)] fn accumulate (& mut self , acc : Cow < 'i , str >) { self . push_str (& acc) ; } }
};
}
