// Generated macro for impl_107 (impl)
macro_rules! Depcrate_page_slotimpl_107 {
() => {
// Module: crate::page::slot
// Provides: {"impl_107"}
// Dependencies: {}
impl < T , C : cfg :: Config > Slot < T , C > { fn release (& self) -> bool { let mut lifecycle = self . lifecycle . load (Ordering :: Acquire) ; loop { let refs = RefCount :: < C > :: from_packed (lifecycle) ; let state = Lifecycle :: < C > :: from_packed (lifecycle) . state ; let gen = LifecycleGen :: < C > :: from_packed (lifecycle) . 0 ; let dropping = refs . value == 1 && state == State :: Marked ; let new_lifecycle = if dropping { LifecycleGen (gen) . pack (State :: Removing as usize) } else { refs . decr () . pack (lifecycle) } ; test_println ! ("-> drop guard: state={:?}; gen={:?}; refs={:?}; lifecycle={:#x}; new_lifecycle={:#x}; dropping={:?}" , state , gen , refs , lifecycle , new_lifecycle , dropping) ; match self . lifecycle . compare_exchange (lifecycle , new_lifecycle , Ordering :: AcqRel , Ordering :: Acquire ,) { Ok (_) => { test_println ! ("-> drop guard: done;  dropping={:?}" , dropping) ; return dropping ; } Err (actual) => { test_println ! ("-> drop guard; retry, actual={:#x}" , actual) ; lifecycle = actual ; } } } } }
};
}
