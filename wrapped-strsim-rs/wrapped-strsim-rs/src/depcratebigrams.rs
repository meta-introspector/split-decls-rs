// Generated macro for bigrams (function)
macro_rules! Depcratebigrams {
() => {
// Module: crate
// Provides: {"bigrams"}
// Dependencies: {}
# [doc = " Returns an Iterator of char tuples."] fn bigrams (s : & str) -> impl Iterator < Item = (char , char) > + '_ { s . chars () . zip (s . chars () . skip (1)) }
};
}
