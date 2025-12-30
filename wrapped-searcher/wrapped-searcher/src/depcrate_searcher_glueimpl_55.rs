// Generated macro for impl_55 (impl)
macro_rules! Depcrate_searcher_glueimpl_55 {
() => {
// Module: crate::searcher::glue
// Provides: {"impl_55"}
// Dependencies: {}
impl < 's , M : Matcher , S : Sink > SliceByLine < 's , M , S > { pub (crate) fn new (searcher : & 's Searcher , matcher : M , slice : & 's [u8] , write_to : S ,) -> SliceByLine < 's , M , S > { debug_assert ! (! searcher . multi_line_with_matcher (& matcher)) ; SliceByLine { core : Core :: new (searcher , matcher , write_to , true) , slice , } } pub (crate) fn run (mut self) -> Result < () , S :: Error > { if self . core . begin () ? { let binary_upto = std :: cmp :: min (self . slice . len () , DEFAULT_BUFFER_CAPACITY) ; let binary_range = Range :: new (0 , binary_upto) ; if ! self . core . detect_binary (self . slice , & binary_range) ? { while ! self . slice [self . core . pos () ..] . is_empty () && self . core . match_by_line (self . slice) ? { } } } let byte_count = self . byte_count () ; let binary_byte_offset = self . core . binary_byte_offset () ; self . core . finish (byte_count , binary_byte_offset) } fn byte_count (& mut self) -> u64 { match self . core . binary_byte_offset () { Some (offset) if offset < self . core . pos () as u64 => offset , _ => self . core . pos () as u64 , } } }
};
}
