// Generated macro for impl_76 (impl)
macro_rules! Depcrate_searcherimpl_76 {
() => {
// Module: crate::searcher
// Provides: {"impl_76"}
// Dependencies: {}
impl std :: fmt :: Display for ConfigError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match * self { ConfigError :: SearchUnavailable => { write ! (f , "grep config error: no available searchers") } ConfigError :: MismatchedLineTerminators { matcher , searcher } => { write ! (f , "grep config error: mismatched line terminators, \
                     matcher has {:?} but searcher has {:?}" , matcher , searcher) } ConfigError :: UnknownEncoding { ref label } => write ! (f , "grep config error: unknown encoding: {}" , String :: from_utf8_lossy (label) ,) , } } }
};
}
