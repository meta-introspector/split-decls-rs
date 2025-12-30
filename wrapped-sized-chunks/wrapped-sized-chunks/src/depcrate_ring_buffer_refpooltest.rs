// Generated macro for test (module)
macro_rules! Depcrate_ring_buffer_refpooltest {
() => {
// Module: crate::ring_buffer::refpool
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use :: refpool :: { Pool , PoolRef } ; use std :: iter :: FromIterator ; # [test] fn default_and_clone () { let pool : Pool < RingBuffer < usize , 64 > > = Pool :: new (16) ; let mut ref1 = PoolRef :: default (& pool) ; { let chunk = PoolRef :: make_mut (& pool , & mut ref1) ; chunk . push_back (1) ; chunk . push_back (2) ; chunk . push_back (3) ; } let ref2 = PoolRef :: cloned (& pool , & ref1) ; let ref3 = PoolRef :: clone_from (& pool , & RingBuffer :: from_iter (1 ..= 3)) ; assert_eq ! (RingBuffer ::< usize , 64 >:: from_iter (1 ..= 3) , * ref1) ; assert_eq ! (RingBuffer ::< usize , 64 >:: from_iter (1 ..= 3) , * ref2) ; assert_eq ! (RingBuffer ::< usize , 64 >:: from_iter (1 ..= 3) , * ref3) ; assert_eq ! (ref1 , ref2) ; assert_eq ! (ref1 , ref3) ; assert_eq ! (ref2 , ref3) ; assert ! (! PoolRef :: ptr_eq (& ref1 , & ref2)) ; } }
};
}
