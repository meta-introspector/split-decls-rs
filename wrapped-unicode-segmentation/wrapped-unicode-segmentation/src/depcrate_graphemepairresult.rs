// Generated macro for PairResult (enum)
macro_rules! Depcrate_graphemePairResult {
() => {
// Module: crate::grapheme
// Provides: {"PairResult"}
// Dependencies: {}
# [derive (PartialEq , Eq)] enum PairResult { # [doc = " definitely not a break"] NotBreak , # [doc = " definitely a break"] Break , # [doc = " a break iff not in extended mode"] Extended , # [doc = " a break unless in extended mode and preceded by"] # [doc = " a sequence of 0 or more InCB=Extend and one or more"] # [doc = " InCB = Linker (in any order),"] # [doc = " preceded by another InCB=Consonant"] InCbConsonant , # [doc = " a break if preceded by an even number of RIS"] Regional , # [doc = " a break if preceded by emoji base and (Extend)*"] Emoji , }
};
}
