// Generated macro for tune (function)
macro_rules! Depcrate_deflatetune {
() => {
// Module: crate::deflate
// Provides: {"tune"}
// Dependencies: {}
pub fn tune (stream : & mut DeflateStream , good_length : usize , max_lazy : usize , nice_length : usize , max_chain : usize ,) -> ReturnCode { stream . state . good_match = good_length as u16 ; stream . state . max_lazy_match = max_lazy as u16 ; stream . state . nice_match = nice_length as u16 ; stream . state . max_chain_length = max_chain as u16 ; ReturnCode :: Ok }
};
}
