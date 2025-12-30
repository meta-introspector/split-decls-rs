// Generated macro for impl_3132 (impl)
macro_rules! Depcrate_sync_mpmc_listimpl_3132 {
() => {
// Module: crate::sync::mpmc::list
// Provides: {"impl_3132"}
// Dependencies: {}
impl < T > Drop for Channel < T > { fn drop (& mut self) { let mut head = self . head . index . load (Ordering :: Relaxed) ; let mut tail = self . tail . index . load (Ordering :: Relaxed) ; let mut block = self . head . block . load (Ordering :: Relaxed) ; head &= ! ((1 << SHIFT) - 1) ; tail &= ! ((1 << SHIFT) - 1) ; unsafe { while head != tail { let offset = (head >> SHIFT) % LAP ; if offset < BLOCK_CAP { let slot = (* block) . slots . get_unchecked (offset) ; let p = & mut * slot . msg . get () ; p . as_mut_ptr () . drop_in_place () ; } else { let next = (* block) . next . load (Ordering :: Relaxed) ; drop (Box :: from_raw (block)) ; block = next ; } head = head . wrapping_add (1 << SHIFT) ; } if ! block . is_null () { drop (Box :: from_raw (block)) ; } } } }
};
}
