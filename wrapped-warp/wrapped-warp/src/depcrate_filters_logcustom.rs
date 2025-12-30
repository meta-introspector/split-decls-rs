// Generated macro for custom (function)
macro_rules! Depcrate_filters_logcustom {
() => {
// Module: crate::filters::log
// Provides: {"custom"}
// Dependencies: {}
# [doc = " Create a wrapping [`Filter`](crate::Filter) that receives `warp::log::Info`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let log = warp::log::custom(|info| {"] # [doc = "     // Use a log macro, or slog, or println, or whatever!"] # [doc = "     eprintln!("] # [doc = "         \"{} {} {}\","] # [doc = "         info.method(),"] # [doc = "         info.path(),"] # [doc = "         info.status(),"] # [doc = "     );"] # [doc = " });"] # [doc = " let route = warp::any()"] # [doc = "     .map(warp::reply)"] # [doc = "     .with(log);"] # [doc = " ```"] pub fn custom < F > (func : F) -> Log < F > where F : Fn (Info < '_ >) , { Log { func } }
};
}
