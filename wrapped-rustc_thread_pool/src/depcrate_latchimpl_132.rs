// Generated macro for impl_132 (impl)
macro_rules! Depcrate_latchimpl_132 {
() => {
// Module: crate::latch
// Provides: {"impl_132"}
// Dependencies: {}
impl CountLatch { pub (super) fn new (owner : Option < & WorkerThread >) -> Self { Self :: with_count (1 , owner) } pub (super) fn with_count (count : usize , owner : Option < & WorkerThread >) -> Self { Self { counter : AtomicUsize :: new (count) , kind : match owner { Some (owner) => CountLatchKind :: Stealing { latch : CoreLatch :: new () , registry : Arc :: clone (owner . registry ()) , worker_index : owner . index () , } , None => CountLatchKind :: Blocking { latch : LockLatch :: new () } , } , } } # [inline] pub (super) fn increment (& self) { let old_counter = self . counter . fetch_add (1 , Ordering :: Relaxed) ; debug_assert ! (old_counter != 0) ; } pub (super) fn wait (& self , owner : Option < & WorkerThread > , all_jobs_started : impl FnMut () -> bool , is_job : impl FnMut (& JobRef) -> bool ,) { match & self . kind { CountLatchKind :: Stealing { latch , registry , worker_index } => unsafe { let owner = owner . expect ("owner thread") ; debug_assert_eq ! (registry . id () , owner . registry () . id ()) ; debug_assert_eq ! (* worker_index , owner . index ()) ; owner . wait_for_jobs :: < _ , true > (latch , all_jobs_started , is_job , | job | { owner . execute (job) ; }) ; } , CountLatchKind :: Blocking { latch } => latch . wait () , } } }
};
}
