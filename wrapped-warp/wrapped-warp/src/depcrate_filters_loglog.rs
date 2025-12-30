// Generated macro for log (function)
macro_rules! Depcrate_filters_loglog {
() => {
// Module: crate::filters::log
// Provides: {"log"}
// Dependencies: {}
# [doc = " Create a wrapping [`Filter`] with the specified `name` as the `target`."] # [doc = ""] # [doc = " This uses the default access logging format, and log records produced"] # [doc = " will have their `target` set to `name`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " // If using something like `pretty_env_logger`,"] # [doc = " // view logs by setting `RUST_LOG=example::api`."] # [doc = " let log = warp::log(\"example::api\");"] # [doc = " let route = warp::any()"] # [doc = "     .map(warp::reply)"] # [doc = "     .with(log);"] # [doc = " ```"] pub fn log (name : & 'static str) -> Log < impl Fn (Info < '_ >) + Copy > { let func = move | info : Info < '_ > | { log :: info ! (target : name , "\"{} {} {:?}\" {} \"{}\" \"{}\" {:?}" , info . method () , info . path () , info . route . version () , info . status () . as_u16 () , OptFmt (info . referer ()) , OptFmt (info . user_agent ()) , info . elapsed () ,) ; } ; Log { func } }
};
}
