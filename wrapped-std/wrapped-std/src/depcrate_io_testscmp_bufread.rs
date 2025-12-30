// Generated macro for cmp_bufread (function)
macro_rules! Depcrate_io_testscmp_bufread {
() => {
// Module: crate::io::tests
// Provides: {"cmp_bufread"}
// Dependencies: {}
fn cmp_bufread < Br1 : BufRead , Br2 : BufRead > (mut br1 : Br1 , mut br2 : Br2 , exp : & [u8]) { let mut cat = Vec :: new () ; loop { let consume = { let buf1 = br1 . fill_buf () . unwrap () ; let buf2 = br2 . fill_buf () . unwrap () ; let minlen = if buf1 . len () < buf2 . len () { buf1 . len () } else { buf2 . len () } ; assert_eq ! (buf1 [.. minlen] , buf2 [.. minlen]) ; cat . extend_from_slice (& buf1 [.. minlen]) ; minlen } ; if consume == 0 { break ; } br1 . consume (consume) ; br2 . consume (consume) ; } assert_eq ! (br1 . fill_buf () . unwrap () . len () , 0) ; assert_eq ! (br2 . fill_buf () . unwrap () . len () , 0) ; assert_eq ! (& cat [..] , & exp [..]) }
};
}
