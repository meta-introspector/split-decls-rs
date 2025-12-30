// Generated macro for tests (module)
macro_rules! Depcrate_stable_vectests {
() => {
// Module: crate::stable_vec
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use { super :: * , memoffset :: offset_of , std :: mem :: { align_of , size_of } , } ; # [test] fn test_memory_layout () { assert_eq ! (offset_of ! (StableVec < i32 >, addr) , 0) ; assert_eq ! (offset_of ! (StableVec < i32 >, cap) , 8) ; assert_eq ! (offset_of ! (StableVec < i32 >, len) , 16) ; assert_eq ! (align_of ::< StableVec < i32 >> () , 8) ; assert_eq ! (size_of ::< StableVec < i32 >> () , 8 + 8 + 8) ; let vec = { let mut vec = Vec :: with_capacity (3) ; vec . push (11) ; vec . push (22) ; vec } ; let vec = StableVec :: from (vec) ; let addr_vec = & vec as * const _ as usize ; let addr_ptr = addr_vec ; let addr_cap = addr_vec + 8 ; let addr_len = addr_vec + 16 ; assert_eq ! (unsafe { * (addr_cap as * const usize) } , 3) ; assert_eq ! (unsafe { * (addr_len as * const usize) } , 2) ; let ptr_data = addr_ptr as * const & [i32 ; 2] ; assert_eq ! (unsafe { * ptr_data } , & [11 , 22]) ; } }
};
}
