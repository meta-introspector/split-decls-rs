// Generated macro for GraphemeClusterBreakTest (struct)
macro_rules! Depcrate_grapheme_cluster_breakGraphemeClusterBreakTest {
() => {
// Module: crate::grapheme_cluster_break
// Provides: {"GraphemeClusterBreakTest"}
// Dependencies: {}
# [doc = " A single row in the `auxiliary/GraphemeBreakTest.txt` file."] # [doc = ""] # [doc = " This file defines tests for the grapheme cluster break algorithm."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct GraphemeClusterBreakTest { # [doc = " Each string is a UTF-8 encoded group of codepoints that make up a"] # [doc = " single grapheme cluster."] pub grapheme_clusters : Vec < String > , # [doc = " A human readable description of this test."] pub comment : String , }
};
}
