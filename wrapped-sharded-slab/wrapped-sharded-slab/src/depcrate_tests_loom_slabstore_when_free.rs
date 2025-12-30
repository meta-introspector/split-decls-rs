// Generated macro for store_when_free (function)
macro_rules! Depcrate_tests_loom_slabstore_when_free {
() => {
// Module: crate::tests::loom_slab
// Provides: {"store_when_free"}
// Dependencies: {}
fn store_when_free < C : crate :: Config > (slab : & Arc < Slab < usize , C > > , t : usize) -> usize { loop { test_println ! ("try store {:?}" , t) ; if let Some (key) = slab . insert (t) { test_println ! ("inserted at {:#x}" , key) ; return key ; } test_println ! ("retrying; slab is full...") ; thread :: yield_now () ; } }
};
}
