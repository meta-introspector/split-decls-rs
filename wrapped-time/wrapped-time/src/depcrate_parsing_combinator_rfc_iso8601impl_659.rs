// Generated macro for impl_659 (impl)
macro_rules! Depcrate_parsing_combinator_rfc_iso8601impl_659 {
() => {
// Module: crate::parsing::combinator::rfc::iso8601
// Provides: {"impl_659"}
// Dependencies: {}
impl ExtendedKind { # [doc = " Is it possible that the format is extended?"] # [inline] pub (crate) const fn maybe_extended (self) -> bool { matches ! (self , Self :: Extended | Self :: Unknown) } # [doc = " Is the format known for certain to be extended?"] # [inline] pub (crate) const fn is_extended (self) -> bool { matches ! (self , Self :: Extended) } # [doc = " If the kind is `Unknown`, make it `Basic`. Otherwise, do nothing. Returns `Some` if and only"] # [doc = " if the kind is now `Basic`."] # [inline] pub (crate) fn coerce_basic (& mut self) -> Option < () > { match self { Self :: Basic => Some (()) , Self :: Extended => None , Self :: Unknown => { * self = Self :: Basic ; Some (()) } } } # [doc = " If the kind is `Unknown`, make it `Extended`. Otherwise, do nothing. Returns `Some` if and"] # [doc = " only if the kind is now `Extended`."] # [inline] pub (crate) fn coerce_extended (& mut self) -> Option < () > { match self { Self :: Basic => None , Self :: Extended => Some (()) , Self :: Unknown => { * self = Self :: Extended ; Some (()) } } } }
};
}
