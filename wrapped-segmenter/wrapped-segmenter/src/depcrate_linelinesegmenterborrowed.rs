// Generated macro for LineSegmenterBorrowed (struct)
macro_rules! Depcrate_lineLineSegmenterBorrowed {
() => {
// Module: crate::line
// Provides: {"LineSegmenterBorrowed"}
// Dependencies: {}
# [doc = " Segments a string into lines (borrowed version)."] # [doc = ""] # [doc = " See [`LineSegmenter`] for examples."] # [derive (Clone , Debug , Copy)] pub struct LineSegmenterBorrowed < 'data > { options : ResolvedLineBreakOptions , data : & 'data RuleBreakData < 'data > , complex : ComplexPayloadsBorrowed < 'data > , }
};
}
