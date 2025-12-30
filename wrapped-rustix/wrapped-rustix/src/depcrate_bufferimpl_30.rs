// Generated macro for impl_30 (impl)
macro_rules! Depcrate_bufferimpl_30 {
() => {
// Module: crate::buffer
// Provides: {"impl_30"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a , T > private :: Sealed < T > for SpareCapacity < 'a , T > { # [doc = " The mutated `Vec` reflects the number of bytes read. We also return"] # [doc = " this number, and a value of 0 indicates the end of the stream has"] # [doc = " been reached."] type Output = usize ; # [inline] fn parts_mut (& mut self) -> (* mut T , usize) { let spare = self . 0 . spare_capacity_mut () ; (spare . as_mut_ptr () . cast () , spare . len ()) } # [inline] unsafe fn assume_init (self , len : usize) -> Self :: Output { self . 0 . set_len (self . 0 . len () + len) ; len } }
};
}
