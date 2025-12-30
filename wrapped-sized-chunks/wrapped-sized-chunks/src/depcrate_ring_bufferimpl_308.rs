// Generated macro for impl_308 (impl)
macro_rules! Depcrate_ring_bufferimpl_308 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_308"}
// Dependencies: {}
impl < A , const N : usize > IndexMut < usize > for RingBuffer < A , N > { # [must_use] fn index_mut (& mut self , index : usize) -> & mut Self :: Output { if index >= self . len () { panic ! ("RingBuffer::index_mut: index out of bounds {} >= {}" , index , self . len ()) ; } unsafe { & mut * self . mut_ptr (self . raw (index)) } } }
};
}
