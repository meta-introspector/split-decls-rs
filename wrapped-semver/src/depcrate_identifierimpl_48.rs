// Generated macro for impl_48 (impl)
macro_rules! Depcrate_identifierimpl_48 {
() => {
// Module: crate::identifier
// Provides: {"impl_48"}
// Dependencies: {}
impl Clone for Identifier { fn clone (& self) -> Self { if self . is_empty_or_inline () { Identifier { head : self . head , tail : self . tail , } } else { let ptr = repr_to_ptr (self . head) ; let len = unsafe { decode_len (ptr) } ; let size = bytes_for_varint (len) + len . get () ; let align = 2 ; let layout = unsafe { Layout :: from_size_align_unchecked (size , align) } ; let clone = unsafe { alloc (layout) } ; if clone . is_null () { handle_alloc_error (layout) ; } unsafe { ptr :: copy_nonoverlapping (ptr , clone , size) } Identifier { head : ptr_to_repr (clone) , tail : [0 ; TAIL_BYTES] , } } } }
};
}
