// Generated macro for impl_22 (impl)
macro_rules! Depcrate_bufferimpl_22 {
() => {
// Module: crate::buffer
// Provides: {"impl_22"}
// Dependencies: {}
impl < T > private :: Sealed < T > for & mut [T] { type Output = usize ; # [inline] fn parts_mut (& mut self) -> (* mut T , usize) { (self . as_mut_ptr () , self . len ()) } # [inline] unsafe fn assume_init (self , len : usize) -> Self :: Output { len } }
};
}
