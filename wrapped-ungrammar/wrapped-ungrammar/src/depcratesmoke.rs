// Generated macro for smoke (function)
macro_rules! Depcratesmoke {
() => {
// Module: crate
// Provides: {"smoke"}
// Dependencies: {}
# [test] fn smoke () { let grammar = include_str ! ("../ungrammar.ungram") ; let grammar = grammar . parse :: < Grammar > () . unwrap () ; drop (grammar) }
};
}
