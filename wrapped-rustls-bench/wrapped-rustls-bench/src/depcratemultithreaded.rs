// Generated macro for multithreaded (function)
macro_rules! Depcratemultithreaded {
() => {
// Module: crate
// Provides: {"multithreaded"}
// Dependencies: {}
# [doc = " Run `f` on `count` threads, and then return the timings produced"] # [doc = " by each thread."] # [doc = ""] # [doc = " `client_config` and `server_config` are cloned into each thread fn."] fn multithreaded (count : NonZeroUsize , client_config : & Arc < ClientConfig > , server_config : & Arc < ServerConfig > , f : impl Fn (Arc < ClientConfig > , Arc < ServerConfig >) -> Timings + Send + Sync ,) -> Vec < Timings > { if count . get () == 1 { return vec ! [f (client_config . clone () , server_config . clone ())] ; } thread :: scope (| s | { let threads = (0 .. count . into ()) . map (| _ | { let client_config = client_config . clone () ; let server_config = server_config . clone () ; s . spawn (| | f (client_config , server_config)) }) . collect :: < Vec < _ > > () ; threads . into_iter () . map (| thread | thread . join () . unwrap ()) . collect :: < Vec < Timings > > () }) }
};
}
