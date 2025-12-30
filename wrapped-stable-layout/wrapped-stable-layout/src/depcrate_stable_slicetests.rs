// Generated macro for tests (module)
macro_rules! Depcrate_stable_slicetests {
() => {
// Module: crate::stable_slice
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: mem :: { align_of , size_of } ; # [test] fn test_memory_layout () { assert_eq ! (align_of ::<& [i32] > () , 8) ; assert_eq ! (size_of ::<& [i32] > () , 8 + 8) ; let array = [11 , 22 , 33 , 44 , 55] ; let slice = array . as_slice () ; let addr_slice = & slice as * const _ as usize ; let addr_ptr = addr_slice ; let addr_len = addr_slice + 8 ; assert_eq ! (unsafe { * (addr_len as * const usize) } , 5) ; let ptr_data = addr_ptr as * const * const i32 ; assert_eq ! (unsafe { * ((* ptr_data) . offset (0)) } , 11) ; assert_eq ! (unsafe { * ((* ptr_data) . offset (1)) } , 22) ; assert_eq ! (unsafe { * ((* ptr_data) . offset (2)) } , 33) ; assert_eq ! (unsafe { * ((* ptr_data) . offset (3)) } , 44) ; assert_eq ! (unsafe { * ((* ptr_data) . offset (4)) } , 55) ; } }
};
}
