// Generated macro for get_nocapture (function)
macro_rules! Depcrate_cliget_nocapture {
() => {
// Module: crate::cli
// Provides: {"get_nocapture"}
// Dependencies: {}
fn get_nocapture (matches : & getopts :: Matches) -> OptPartRes < bool > { let mut nocapture = matches . opt_present ("nocapture") || matches . opt_present ("no-capture") ; if ! nocapture { nocapture = match env :: var ("RUST_TEST_NOCAPTURE") { Ok (val) => & val != "0" , Err (_) => false , } ; } Ok (nocapture) }
};
}
