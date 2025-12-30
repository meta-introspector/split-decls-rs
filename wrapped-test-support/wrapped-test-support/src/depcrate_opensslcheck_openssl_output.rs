// Generated macro for check_openssl_output (function)
macro_rules! Depcrate_opensslcheck_openssl_output {
() => {
// Module: crate::openssl
// Provides: {"check_openssl_output"}
// Dependencies: {}
fn check_openssl_output (command_and_args : & [& str] , pem : & [u8]) -> String { let tmp_dir = tempdir () . expect ("create tempdir") ; let cert_path = tmp_dir . path () . join ("cert.pem") ; let mut cert_file = File :: create (& cert_path) . expect ("create pem file") ; cert_file . write_all (pem) . expect ("Create pem file") ; let mut child = Command :: new ("openssl") . args (command_and_args) . arg ("-in") . arg (& cert_path) . arg ("-noout") . arg ("-text") . stderr (Stdio :: inherit ()) . stdout (Stdio :: piped ()) . spawn () . expect ("openssl failed") ; let mut stdout = child . stdout . take () . unwrap () ; let exit_status = child . wait () . expect ("get openssl x509 status") ; assert ! (exit_status . success () , "openssl failed") ; let mut output_buf = Vec :: new () ; stdout . read_to_end (& mut output_buf) . expect ("read openssl output") ; String :: from_utf8 (output_buf . clone ()) . unwrap () }
};
}
