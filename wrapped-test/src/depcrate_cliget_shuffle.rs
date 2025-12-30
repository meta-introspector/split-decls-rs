// Generated macro for get_shuffle (function)
macro_rules! Depcrate_cliget_shuffle {
() => {
// Module: crate::cli
// Provides: {"get_shuffle"}
// Dependencies: {}
fn get_shuffle (matches : & getopts :: Matches , allow_unstable : bool) -> OptPartRes < bool > { let mut shuffle = unstable_optflag ! (matches , allow_unstable , "shuffle") ; if ! shuffle && allow_unstable { shuffle = match env :: var ("RUST_TEST_SHUFFLE") { Ok (val) => & val != "0" , Err (_) => false , } ; } Ok (shuffle) }
};
}
