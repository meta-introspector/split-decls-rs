// Generated macro for get_shuffle_seed (function)
macro_rules! Depcrate_cliget_shuffle_seed {
() => {
// Module: crate::cli
// Provides: {"get_shuffle_seed"}
// Dependencies: {}
fn get_shuffle_seed (matches : & getopts :: Matches , allow_unstable : bool) -> OptPartRes < Option < u64 > > { let mut shuffle_seed = match unstable_optopt ! (matches , allow_unstable , "shuffle-seed") { Some (n_str) => match n_str . parse :: < u64 > () { Ok (n) => Some (n) , Err (e) => { return Err (format ! ("argument for --shuffle-seed must be a number \
                     (error: {e})")) ; } } , None => None , } ; if shuffle_seed . is_none () && allow_unstable { shuffle_seed = match env :: var ("RUST_TEST_SHUFFLE_SEED") { Ok (val) => match val . parse :: < u64 > () { Ok (n) => Some (n) , Err (_) => panic ! ("RUST_TEST_SHUFFLE_SEED is `{val}`, should be a number.") , } , Err (_) => None , } ; } Ok (shuffle_seed) }
};
}
