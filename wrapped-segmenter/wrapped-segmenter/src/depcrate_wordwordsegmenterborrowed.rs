// Generated macro for WordSegmenterBorrowed (struct)
macro_rules! Depcrate_wordWordSegmenterBorrowed {
() => {
// Module: crate::word
// Provides: {"WordSegmenterBorrowed"}
// Dependencies: {}
# [doc = " Segments a string into words (borrowed version)."] # [doc = ""] # [doc = " See [`WordSegmenter`] for examples."] # [derive (Clone , Debug , Copy)] pub struct WordSegmenterBorrowed < 'data > { data : & 'data RuleBreakData < 'data > , complex : ComplexPayloadsBorrowed < 'data > , locale_override : Option < & 'data RuleBreakDataOverride < 'data > > , }
};
}
