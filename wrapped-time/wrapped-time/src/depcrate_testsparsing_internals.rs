// Generated macro for parsing_internals (function)
macro_rules! Depcrate_testsparsing_internals {
() => {
// Module: crate::tests
// Provides: {"parsing_internals"}
// Dependencies: {}
# [test] fn parsing_internals () { assert ! (parsing :: ParsedItem (b"" , ()) . flat_map (| _ | None ::< () >) . is_none ()) ; assert ! (< NonZero < u8 > as Integer >:: parse_bytes (b"256") . is_none ()) ; }
};
}
