// Generated macro for test_add_12 (function)
macro_rules! Depcrate_blocktest_add_12 {
() => {
// Module: crate::block
// Provides: {"test_add_12"}
// Dependencies: {}
# [test] fn test_add_12 () { # [track_caller] fn invoke_assert (block : & Add12 , expected : i32) { assert_eq ! (block . call ((1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12)) , expected) ; assert_eq ! (unsafe { invoke_add_12 (block , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12) } , expected) ; } struct Enc ; unsafe impl ManualBlockEncoding for Enc { type Arguments = (i32 , i32 , i32 , i32 , i32 , i32 , i32 , i32 , i32 , i32 , i32 , i32) ; type Return = i32 ; # [cfg (target_pointer_width = "64")] const ENCODING_CSTR : & 'static CStr = c"i56@?0i8i12i16i20i24i28i32i36i40i44i48i52" ; # [cfg (target_pointer_width = "32")] const ENCODING_CSTR : & 'static CStr = c"i52@?0i4i8i12i16i20i24i28i32i36i40i44i48" ; } global_block ! { static GLOBAL_BLOCK = | a1 : i32 , a2 : i32 , a3 : i32 , a4 : i32 , a5 : i32 , a6 : i32 , a7 : i32 , a8 : i32 , a9 : i32 , a10 : i32 , a11 : i32 , a12 : i32 , | -> i32 { a1 + a2 + a3 + a4 + a5 + a6 + a7 + a8 + a9 + a10 + a11 + a12 + 42 } ; } invoke_assert (unsafe { & * get_add_12 () } , 78) ; invoke_assert (unsafe { & * get_add_12_with (13) } , 91) ; let closure = | a1 , a2 , a3 , a4 , a5 , a6 , a7 , a8 , a9 , a10 , a11 , a12 | { a1 + a2 + a3 + a4 + a5 + a6 + a7 + a8 + a9 + a10 + a11 + a12 } ; invoke_assert (& StackBlock :: new (closure) , 78) ; invoke_assert (& RcBlock :: new (closure) , 78) ; invoke_assert (& StackBlock :: with_encoding :: < Enc > (closure) , 78) ; invoke_assert (& RcBlock :: with_encoding :: < _ , _ , _ , Enc > (closure) , 78) ; invoke_assert (& GLOBAL_BLOCK , 120) ; }
};
}
