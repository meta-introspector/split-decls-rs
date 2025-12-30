// Generated macro for uncompress_help (function)
macro_rules! Depcrate_inflateuncompress_help {
() => {
// Module: crate::inflate
// Provides: {"uncompress_help"}
// Dependencies: {}
fn uncompress_help (input : & [u8]) -> Vec < u8 > { use libz_rs_sys :: * ; let mut dest_vec = vec ! [0u8 ; 1 << 16] ; let mut dest_len = dest_vec . len () as c_ulong ; let dest = dest_vec . as_mut_ptr () ; let source = input . as_ptr () ; let source_len = input . len () as _ ; let err = unsafe { uncompress (dest , & mut dest_len , source , source_len) } ; if err != 0 { panic ! ("error {:?}" , ReturnCode :: from (err)) ; } dest_vec . truncate (dest_len as usize) ; dest_vec }
};
}
