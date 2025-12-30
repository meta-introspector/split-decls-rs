// Generated macro for modified_test (function)
macro_rules! Depcrate_testmodified_test {
() => {
// Module: crate::test
// Provides: {"modified_test"}
// Dependencies: {}
# [test] fn modified_test () { init_log () ; use std :: io :: BufRead ; let filename = "tests/writemode/source/modified.rs" ; let mut data = Vec :: new () ; let mut config = Config :: default () ; config . set () . emit_mode (crate :: config :: EmitMode :: ModifiedLines) ; { let mut session = Session :: new (config , Some (& mut data)) ; session . format (Input :: File (filename . into ())) . unwrap () ; } let mut lines = data . lines () ; let mut chunks = Vec :: new () ; while let Some (Ok (header)) = lines . next () { let values : Vec < _ > = header . split (' ') . map (| s | s . parse :: < u32 > () . unwrap ()) . collect () ; assert_eq ! (values . len () , 3) ; let line_number_orig = values [0] ; let lines_removed = values [1] ; let num_added = values [2] ; let mut added_lines = Vec :: new () ; for _ in 0 .. num_added { added_lines . push (lines . next () . unwrap () . unwrap ()) ; } chunks . push (ModifiedChunk { line_number_orig , lines_removed , lines : added_lines , }) ; } assert_eq ! (chunks , vec ! [ModifiedChunk { line_number_orig : 4 , lines_removed : 4 , lines : vec ! ["fn blah() {}" . into ()] , } , ModifiedChunk { line_number_orig : 9 , lines_removed : 6 , lines : vec ! ["#[cfg(a, b)]" . into () , "fn main() {}" . into ()] , } ,] ,) ; }
};
}
