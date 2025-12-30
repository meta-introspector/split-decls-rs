// Generated macro for SearcherTester (struct)
macro_rules! Depcrate_testutilSearcherTester {
() => {
// Module: crate::testutil
// Provides: {"SearcherTester"}
// Dependencies: {}
# [doc = " A type for expressing tests on a searcher."] # [doc = ""] # [doc = " The searcher code has a lot of different code paths, mostly for the"] # [doc = " purposes of optimizing a bunch of different use cases. The intent of the"] # [doc = " searcher is to pick the best code path based on the configuration, which"] # [doc = " means there is no obviously direct way to ask that a specific code path"] # [doc = " be exercised. Thus, the purpose of this tester is to explicitly check as"] # [doc = " many code paths that make sense."] # [doc = ""] # [doc = " The tester works by assuming you want to test all pertinent code paths."] # [doc = " These can be trimmed down as necessary via the various builder methods."] # [derive (Debug)] pub (crate) struct SearcherTester { haystack : String , pattern : String , filter : Option < :: regex :: Regex > , print_labels : bool , expected_no_line_number : Option < String > , expected_with_line_number : Option < String > , expected_slice_no_line_number : Option < String > , expected_slice_with_line_number : Option < String > , by_line : bool , multi_line : bool , invert_match : bool , line_number : bool , binary : BinaryDetection , auto_heap_limit : bool , after_context : usize , before_context : usize , passthru : bool , }
};
}
