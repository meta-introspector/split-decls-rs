// Generated macro for check_certificate (function)
macro_rules! Depcrate_zlintcheck_certificate {
() => {
// Module: crate::zlint
// Provides: {"check_certificate"}
// Dependencies: {}
pub fn check_certificate (pem : & [u8] , ignored : & [& str]) { let tmp_dir = tempdir () . expect ("create tempdir") ; let config_path = tmp_dir . path () . join ("zlint_config.toml") ; let cert_path = tmp_dir . path () . join ("zlint_cert.pem") ; { let mut config_file = File :: create (& config_path) . expect ("create config file") ; config_file . write_all (ZLINT_CONFIG . as_bytes ()) . expect ("Create config file") ; } { let mut cert_file = File :: create (& cert_path) . expect ("create pem file") ; cert_file . write_all (pem) . expect ("Create pem file") ; } let mut child = Command :: new ("zlint") . arg ("-pretty") . arg ("-config") . arg (& config_path) . arg (& cert_path) . stderr (Stdio :: inherit ()) . stdout (Stdio :: piped ()) . spawn () . unwrap_or_else (| e | match e . kind () { io :: ErrorKind :: NotFound => { panic ! ("error running 'zlint': command not found. Is it installed?") } _ => panic ! ("error running 'zlint': {e:?}") , }) ; let mut stdout = child . stdout . take () . unwrap () ; let output_buf = { let mut buf = Vec :: new () ; stdout . read_to_end (& mut buf) . expect ("read zlint output") ; buf } ; let exit_status = child . wait () . expect ("get zlint status") ; assert ! (exit_status . success () , "zlint failed") ; let output : LintResult = serde_json :: from_slice (& output_buf) . expect ("parse zlint output") ; assert ! (output . check_lints (ignored)) ; }
};
}
