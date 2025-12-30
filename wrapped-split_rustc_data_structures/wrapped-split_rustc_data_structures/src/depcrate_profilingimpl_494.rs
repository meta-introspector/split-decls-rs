// Generated macro for impl_494 (impl)
macro_rules! Depcrate_profilingimpl_494 {
() => {
// Module: crate::profiling
// Provides: {"impl_494"}
// Dependencies: {}
impl Display for JsonTimePassesEntry < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let Self { pass : what , time , start_rss , end_rss } = self ; write ! (f , r#"{{"pass":"{what}","time":{time},"rss_start":"#) . unwrap () ; match start_rss { Some (rss) => write ! (f , "{rss}") ? , None => write ! (f , "null") ? , } write ! (f , r#","rss_end":"#) ? ; match end_rss { Some (rss) => write ! (f , "{rss}") ? , None => write ! (f , "null") ? , } write ! (f , "}}") ? ; Ok (()) } }
};
}
