// Generated macro for get_shuffle_seed (function)
macro_rules! Depcrate_helpers_shuffleget_shuffle_seed {
() => {
// Module: crate::helpers::shuffle
// Provides: {"get_shuffle_seed"}
// Dependencies: {}
pub (crate) fn get_shuffle_seed (opts : & TestOpts) -> Option < u64 > { opts . shuffle_seed . or_else (| | { if opts . shuffle { Some (SystemTime :: now () . duration_since (UNIX_EPOCH) . expect ("Failed to get system time") . as_nanos () as u64 ,) } else { None } }) }
};
}
