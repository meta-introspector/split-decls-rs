// Generated macro for impl_24 (impl)
macro_rules! Depcrate_bufferimpl_24 {
() => {
// Module: crate::buffer
// Provides: {"impl_24"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T > private :: Sealed < T > for & mut Vec < T > { type Output = usize ; # [inline] fn parts_mut (& mut self) -> (* mut T , usize) { (self . as_mut_ptr () , self . len ()) } # [inline] unsafe fn assume_init (self , len : usize) -> Self :: Output { len } }
};
}
