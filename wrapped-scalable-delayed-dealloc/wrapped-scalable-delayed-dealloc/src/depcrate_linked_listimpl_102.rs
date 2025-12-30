// Generated macro for impl_102 (impl)
macro_rules! Depcrate_linked_listimpl_102 {
() => {
// Module: crate::linked_list
// Provides: {"impl_102"}
// Dependencies: {}
impl < T > Drop for LinkedEntry < T > { # [inline] fn drop (& mut self) { if ! self . next . is_null (Relaxed) { let guard = Guard :: new () ; if let Some (next_entry) = self . next . load (Relaxed , & guard) . as_ref () { next_ptr_recursive (next_entry , Relaxed , 64 , & guard) ; } } } }
};
}
