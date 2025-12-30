// Generated macro for verify (function)
macro_rules! Depcrate_opensslverify {
() => {
// Module: crate::openssl
// Provides: {"verify"}
// Dependencies: {}
pub fn verify (trust_anchor : & [u8] , leaf : & [u8] , crl : & [u8]) -> (ExitStatus , String , String) { let tmp_dir = tempdir () . expect ("create tempdir") ; let trust_anchor_path = tmp_dir . path () . join ("trust_anchor.pem") ; let leaf_path = tmp_dir . path () . join ("leaf.pem") ; let crl_path = tmp_dir . path () . join ("crl.pem") ; fs :: write (& trust_anchor_path , trust_anchor) . expect ("Write trust anchor") ; fs :: write (& leaf_path , leaf) . expect ("Write leaf") ; fs :: write (& crl_path , crl) . expect ("Write crl") ; let mut child = Command :: new ("openssl") . arg ("verify") . arg ("-crl_check") . arg ("-CRLfile") . arg (& crl_path) . arg ("-trusted") . arg (& trust_anchor_path) . arg ("--") . arg (& leaf_path) . stderr (Stdio :: piped ()) . stdout (Stdio :: piped ()) . spawn () . expect ("openssl failed") ; let mut stdout = child . stdout . take () . unwrap () ; let mut stderr = child . stderr . take () . unwrap () ; let mut output_buf = Vec :: new () ; stdout . read_to_end (& mut output_buf) . expect ("read openssl output") ; let mut stderr_buf = Vec :: new () ; stderr . read_to_end (& mut stderr_buf) . expect ("read openssl output") ; let exit_status = child . wait () . expect ("get openssl verify status") ; (exit_status , String :: from_utf8 (output_buf . clone ()) . unwrap () , String :: from_utf8 (stderr_buf . clone ()) . unwrap () ,) }
};
}
