// Generated macro for SentenceSegmenterBorrowed (struct)
macro_rules! Depcrate_sentenceSentenceSegmenterBorrowed {
() => {
// Module: crate::sentence
// Provides: {"SentenceSegmenterBorrowed"}
// Dependencies: {}
# [doc = " Segments a string into sentences (borrowed version)."] # [doc = ""] # [doc = " See [`SentenceSegmenter`] for examples."] # [derive (Clone , Debug , Copy)] pub struct SentenceSegmenterBorrowed < 'data > { data : & 'data RuleBreakData < 'data > , locale_override : Option < & 'data RuleBreakDataOverride < 'data > > , }
};
}
