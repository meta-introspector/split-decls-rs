// Generated macro for copy_direct_from_next_in_to_next_out (function)
macro_rules! Depcrate_inflatecopy_direct_from_next_in_to_next_out {
() => {
// Module: crate::inflate
// Provides: {"copy_direct_from_next_in_to_next_out"}
// Dependencies: {}
# [test] fn copy_direct_from_next_in_to_next_out () { assert_eq_rs_ng ! ({ let input = [120 , 1 , 1 , 2 , 0 , 253 , 255 , 6 , 10 , 0 , 24 , 0 , 17] ; let mut dest_vec = vec ! [0u8 ; 1 << 16] ; let mut dest_len = dest_vec . len () as c_ulong ; let dest = dest_vec . as_mut_ptr () ; let source = input . as_ptr () ; let source_len = input . len () as _ ; let err = unsafe { uncompress (dest , & mut dest_len , source , source_len) } ; if err != 0 { panic ! ("error {:?}" , ReturnCode :: from (err)) ; } dest_vec . truncate (dest_len as usize) ; assert_eq ! (String :: from_utf8 (dest_vec) . unwrap () , "\u{6}\n") ; }) ; }
};
}
