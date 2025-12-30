// Generated macro for tests (module)
macro_rules! Depcrate_searchertests {
() => {
// Module: crate::searcher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: testutil :: { KitchenSink , RegexMatcher } ; use super :: * ; # [test] fn config_error_heap_limit () { let matcher = RegexMatcher :: new ("") ; let sink = KitchenSink :: new () ; let mut searcher = SearcherBuilder :: new () . heap_limit (Some (0)) . build () ; let res = searcher . search_slice (matcher , & [] , sink) ; assert ! (res . is_err ()) ; } # [test] fn config_error_line_terminator () { let mut matcher = RegexMatcher :: new ("") ; matcher . set_line_term (Some (LineTerminator :: byte (b'z'))) ; let sink = KitchenSink :: new () ; let mut searcher = Searcher :: new () ; let res = searcher . search_slice (matcher , & [] , sink) ; assert ! (res . is_err ()) ; } # [test] fn uft8_bom_sniffing () { let matcher = RegexMatcher :: new ("foo") ; let haystack : & [u8] = & [0xef , 0xbb , 0xbf , 0x66 , 0x6f , 0x6f] ; let mut sink = KitchenSink :: new () ; let mut searcher = SearcherBuilder :: new () . build () ; let res = searcher . search_slice (matcher , haystack , & mut sink) ; assert ! (res . is_ok ()) ; let sink_output = String :: from_utf8 (sink . as_bytes () . to_vec ()) . unwrap () ; assert_eq ! (sink_output , "1:0:foo\nbyte count:3\n") ; } }
};
}
