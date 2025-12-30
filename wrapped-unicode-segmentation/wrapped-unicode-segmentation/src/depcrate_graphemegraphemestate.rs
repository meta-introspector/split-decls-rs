// Generated macro for GraphemeState (enum)
macro_rules! Depcrate_graphemeGraphemeState {
() => {
// Module: crate::grapheme
// Provides: {"GraphemeState"}
// Dependencies: {}
# [doc = " maybe unify with PairResult?"] # [doc = " An enum describing information about a potential boundary."] # [derive (PartialEq , Eq , Clone , Debug)] enum GraphemeState { # [doc = " No information is known."] Unknown , # [doc = " It is known to not be a boundary."] NotBreak , # [doc = " It is known to be a boundary."] Break , # [doc = " The codepoint after it has Indic_Conjunct_Break=Consonant,"] # [doc = " so there is a break before so a boundary if it is preceded by another"] # [doc = " InCB=Consonant follwoed by a sequence consisting of one or more InCB=Linker"] # [doc = " and zero or more InCB = Extend (in any order)."] InCbConsonant , # [doc = " The codepoint after is a Regional Indicator Symbol, so a boundary iff"] # [doc = " it is preceded by an even number of RIS codepoints. (GB12, GB13)"] Regional , # [doc = " The codepoint after is Extended_Pictographic,"] # [doc = " so whether it's a boundary depends on pre-context according to GB11."] Emoji , }
};
}
