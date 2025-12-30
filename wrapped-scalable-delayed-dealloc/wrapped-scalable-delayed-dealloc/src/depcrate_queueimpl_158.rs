// Generated macro for impl_158 (impl)
macro_rules! Depcrate_queueimpl_158 {
() => {
// Module: crate::queue
// Provides: {"impl_158"}
// Dependencies: {}
impl < 'g , T > Iterator for Iter < 'g , T > { type Item = & 'g T ; # [inline] fn next (& mut self) -> Option < Self :: Item > { if let Some (current) = self . current . as_ref () { self . current = current . next_ptr (Acquire , self . guard) ; Some (current) } else { None } } }
};
}
