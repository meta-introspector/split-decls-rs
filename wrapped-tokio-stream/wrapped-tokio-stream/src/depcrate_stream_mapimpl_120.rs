// Generated macro for impl_120 (impl)
macro_rules! Depcrate_stream_mapimpl_120 {
() => {
// Module: crate::stream_map
// Provides: {"impl_120"}
// Dependencies: {}
impl < K , V > StreamMap < K , V > where K : Unpin , V : Stream + Unpin , { # [doc = " Polls the next value, includes the vec entry index"] fn poll_next_entry (& mut self , cx : & mut Context < '_ >) -> Poll < Option < (usize , V :: Item) > > { let start = self :: rand :: thread_rng_n (self . entries . len () as u32) as usize ; let mut idx = start ; for _ in 0 .. self . entries . len () { let (_ , stream) = & mut self . entries [idx] ; match Pin :: new (stream) . poll_next (cx) { Poll :: Ready (Some (val)) => return Poll :: Ready (Some ((idx , val))) , Poll :: Ready (None) => { self . entries . swap_remove (idx) ; if idx == self . entries . len () { idx = 0 ; } else if idx < start && start <= self . entries . len () { idx = idx . wrapping_add (1) % self . entries . len () ; } } Poll :: Pending => { idx = idx . wrapping_add (1) % self . entries . len () ; } } } if self . entries . is_empty () { Poll :: Ready (None) } else { Poll :: Pending } } }
};
}
