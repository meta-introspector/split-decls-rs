// Generated macro for impl_25 (impl)
macro_rules! Depcrate_bufferimpl_25 {
() => {
// Module: crate::buffer
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'a , T > private :: Sealed < T > for & 'a mut [MaybeUninit < T >] { type Output = (& 'a mut [T] , & 'a mut [MaybeUninit < T >]) ; # [inline] fn parts_mut (& mut self) -> (* mut T , usize) { (self . as_mut_ptr () . cast () , self . len ()) } # [inline] unsafe fn assume_init (self , len : usize) -> Self :: Output { let (init , uninit) = self . split_at_mut (len) ; let init = slice :: from_raw_parts_mut (init . as_mut_ptr () . cast :: < T > () , init . len ()) ; (init , uninit) } }
};
}
