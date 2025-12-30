// Generated macro for SentenceBreakTest (struct)
macro_rules! Depcrate_sentence_breakSentenceBreakTest {
() => {
// Module: crate::sentence_break
// Provides: {"SentenceBreakTest"}
// Dependencies: {}
# [doc = " A single row in the `auxiliary/SentenceBreakTest.txt` file."] # [doc = ""] # [doc = " This file defines tests for the sentence break algorithm."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct SentenceBreakTest { # [doc = " Each string is a UTF-8 encoded group of codepoints that make up a"] # [doc = " single sentence."] pub sentences : Vec < String > , # [doc = " A human readable description of this test."] pub comment : String , }
};
}
