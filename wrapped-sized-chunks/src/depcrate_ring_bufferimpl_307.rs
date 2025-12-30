// Generated macro for impl_307 (impl)
macro_rules! Depcrate_ring_bufferimpl_307 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_307"}
// Dependencies: {}
impl < A , const N : usize > Index < usize > for RingBuffer < A , N > { type Output = A ; # [must_use] fn index (& self , index : usize) -> & Self :: Output { if index >= self . len () { panic ! ("RingBuffer::index: index out of bounds {} >= {}" , index , self . len ()) ; } unsafe { & * self . ptr (self . raw (index)) } } }
};
}
