// Generated macro for Core (struct)
macro_rules! Depcrate_searcher_coreCore {
() => {
// Module: crate::searcher::core
// Provides: {"Core"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct Core < 's , M : 's , S > { config : & 's Config , matcher : M , searcher : & 's Searcher , sink : S , binary : bool , pos : usize , absolute_byte_offset : u64 , binary_byte_offset : Option < usize > , line_number : Option < u64 > , last_line_counted : usize , last_line_visited : usize , after_context_left : usize , has_sunk : bool , has_matched : bool , count : u64 , }
};
}
