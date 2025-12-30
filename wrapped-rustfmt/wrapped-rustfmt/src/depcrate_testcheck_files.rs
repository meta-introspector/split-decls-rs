// Generated macro for check_files (function)
macro_rules! Depcrate_testcheck_files {
() => {
// Module: crate::test
// Provides: {"check_files"}
// Dependencies: {}
fn check_files (files : Vec < PathBuf > , opt_config : & Option < PathBuf >) -> (Vec < FormatReport > , u32 , u32) { let mut count = 0 ; let mut fails = 0 ; let mut reports = vec ! [] ; for file_name in files { let sig_comments = read_significant_comments (& file_name) ; if sig_comments . contains_key ("unstable") && ! is_nightly_channel ! () { debug ! ("Skipping '{}' because it requires unstable \
                 features which are only available on nightly..." , file_name . display ()) ; continue ; } debug ! ("Testing '{}'..." , file_name . display ()) ; match idempotent_check (& file_name , opt_config) { Ok (ref report) if report . has_warnings () => { print ! ("{}" , FormatReportFormatterBuilder :: new (report) . build ()) ; fails += 1 ; } Ok (report) => reports . push (report) , Err (err) => { if let IdempotentCheckError :: Mismatch (msg) = err { print_mismatches_default_message (msg) ; } fails += 1 ; } } count += 1 ; } (reports , count , fails) }
};
}
