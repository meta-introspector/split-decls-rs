// Generated macro for impl_116 (impl)
macro_rules! Depcrate_testutilimpl_116 {
() => {
// Module: crate::testutil
// Provides: {"impl_116"}
// Dependencies: {}
impl TesterConfig { # [doc = " Execute a search using a reader. This exercises the incremental search"] # [doc = " strategy, where the entire contents of the corpus aren't necessarily"] # [doc = " in memory at once."] fn search_reader (& self , haystack : & str) -> String { let mut sink = KitchenSink :: new () ; let mut searcher = self . builder . build () ; let result = searcher . search_reader (& self . matcher , haystack . as_bytes () , & mut sink ,) ; if let Err (err) = result { let label = format ! ("reader-{}" , self . label) ; panic ! ("error running '{}': {}" , label , err) ; } String :: from_utf8 (sink . as_bytes () . to_vec ()) . unwrap () } # [doc = " Execute a search using a slice. This exercises the search routines that"] # [doc = " have the entire contents of the corpus in memory at one time."] fn search_slice (& self , haystack : & str) -> String { let mut sink = KitchenSink :: new () ; let mut searcher = self . builder . build () ; let result = searcher . search_slice (& self . matcher , haystack . as_bytes () , & mut sink ,) ; if let Err (err) = result { let label = format ! ("slice-{}" , self . label) ; panic ! ("error running '{}': {}" , label , err) ; } String :: from_utf8 (sink . as_bytes () . to_vec ()) . unwrap () } }
};
}
