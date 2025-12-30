// Generated macro for impl_109 (impl)
macro_rules! Depcrate_testutilimpl_109 {
() => {
// Module: crate::testutil
// Provides: {"impl_109"}
// Dependencies: {}
impl Matcher for RegexMatcher { type Captures = NoCaptures ; type Error = NoError ; fn find_at (& self , haystack : & [u8] , at : usize ,) -> Result < Option < Match > , NoError > { Ok (self . regex . find_at (haystack , at) . map (| m | Match :: new (m . start () , m . end ()))) } fn new_captures (& self) -> Result < NoCaptures , NoError > { Ok (NoCaptures :: new ()) } fn line_terminator (& self) -> Option < LineTerminator > { self . line_term } fn find_candidate_line (& self , haystack : & [u8] ,) -> Result < Option < LineMatchKind > , NoError > { if self . every_line_is_candidate { assert ! (self . line_term . is_some ()) ; if haystack . is_empty () { return Ok (None) ; } let i = haystack . find_byte (self . line_term . unwrap () . as_byte ()) . map (| i | i) . unwrap_or (haystack . len () - 1) ; Ok (Some (LineMatchKind :: Candidate (i))) } else { Ok (self . shortest_match (haystack) ? . map (LineMatchKind :: Confirmed)) } } }
};
}
