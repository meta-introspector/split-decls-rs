// Generated macro for thread_runner (function)
macro_rules! Depcrate_thread_atomics_spawnthread_runner {
() => {
// Module: crate::thread::atomics::spawn
// Provides: {"thread_runner"}
// Dependencies: {}
# [doc = " Common functionality between threads, regardless if a message is passed."] fn thread_runner < 'scope , T : 'scope + Send , F1 : 'scope + FnOnce () -> F2 , F2 : Future < Output = T > > (thread : Thread , stack_size : Option < usize > , result_sender : oneshot :: Sender < T > , # [cfg (feature = "message")] spawn_sender : channel :: Sender < SpawnData > , scope : Option < Arc < ScopeData > > , task : F1 ,) -> Pin < Box < dyn 'scope + Future < Output = u32 > > > { Box :: pin (async move { Thread :: register (thread) ; # [cfg (feature = "message")] { let old = SPAWN_SENDER . with (| cell | cell . borrow_mut () . replace (spawn_sender)) ; debug_assert ! (old . is_none () , "found existing `Sender` in new thread") ; } result_sender . send (task () . await) ; if let Some (scope) = scope { if scope . threads . fetch_sub (1 , Ordering :: Release) == 1 { scope . thread . unpark () ; scope . waker . wake () ; } } # [cfg (feature = "message")] SPAWN_SENDER . with (| cell | cell . borrow_mut () . take ()) . expect ("found no `Sender` in existing thread") ; let value = Box :: pin (AtomicI32 :: new (0)) ; let index = super :: i32_to_buffer_index (value . as_ptr ()) ; Command :: Terminate { id : super :: current_id () , value , memory : ThreadMemory :: new (stack_size) , } . send () ; index }) }
};
}
