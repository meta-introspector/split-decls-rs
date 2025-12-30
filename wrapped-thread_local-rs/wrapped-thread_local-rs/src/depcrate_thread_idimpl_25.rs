// Generated macro for impl_25 (impl)
macro_rules! Depcrate_thread_idimpl_25 {
() => {
// Module: crate::thread_id
// Provides: {"impl_25"}
// Dependencies: {}
impl ThreadIdManager { const fn new () -> Self { Self { free_from : 0 , free_list : None , } } fn alloc (& mut self) -> usize { if let Some (id) = self . free_list . as_mut () . and_then (| heap | heap . pop ()) { id . 0 } else { let id = self . free_from ; self . free_from += 1 ; id } } fn free (& mut self , id : usize) { self . free_list . get_or_insert_with (BinaryHeap :: new) . push (Reverse (id)) ; } }
};
}
