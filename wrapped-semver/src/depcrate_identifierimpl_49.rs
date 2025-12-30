// Generated macro for impl_49 (impl)
macro_rules! Depcrate_identifierimpl_49 {
() => {
// Module: crate::identifier
// Provides: {"impl_49"}
// Dependencies: {}
impl Drop for Identifier { fn drop (& mut self) { if self . is_empty_or_inline () { return ; } let ptr = repr_to_ptr_mut (self . head) ; let len = unsafe { decode_len (ptr) } ; let size = bytes_for_varint (len) + len . get () ; let align = 2 ; let layout = unsafe { Layout :: from_size_align_unchecked (size , align) } ; unsafe { dealloc (ptr , layout) } } }
};
}
