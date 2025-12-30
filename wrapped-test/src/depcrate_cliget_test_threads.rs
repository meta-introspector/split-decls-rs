// Generated macro for get_test_threads (function)
macro_rules! Depcrate_cliget_test_threads {
() => {
// Module: crate::cli
// Provides: {"get_test_threads"}
// Dependencies: {}
fn get_test_threads (matches : & getopts :: Matches) -> OptPartRes < Option < usize > > { let test_threads = match matches . opt_str ("test-threads") { Some (n_str) => match n_str . parse :: < usize > () { Ok (0) => return Err ("argument for --test-threads must not be 0" . to_string ()) , Ok (n) => Some (n) , Err (e) => { return Err (format ! ("argument for --test-threads must be a number > 0 \
                     (error: {e})")) ; } } , None => None , } ; Ok (test_threads) }
};
}
