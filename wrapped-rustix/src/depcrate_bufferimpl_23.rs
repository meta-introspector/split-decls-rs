// Generated macro for impl_23 (impl)
macro_rules! Depcrate_bufferimpl_23 {
() => {
// Module: crate::buffer
// Provides: {"impl_23"}
// Dependencies: {}
impl < T , const N : usize > private :: Sealed < T > for & mut [T ; N] { type Output = usize ; # [inline] fn parts_mut (& mut self) -> (* mut T , usize) { (self . as_mut_ptr () , N) } # [inline] unsafe fn assume_init (self , len : usize) -> Self :: Output { len } }
};
}
