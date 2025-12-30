// Generated macro for impl_28 (impl)
macro_rules! Depcrate_thread_idimpl_28 {
() => {
// Module: crate::thread_id
// Provides: {"impl_28"}
// Dependencies: {}
impl Thread { pub (crate) fn new (id : usize) -> Self { let bucket = (usize :: BITS as usize) - ((id + 1) . leading_zeros () as usize) - 1 ; let bucket_size = 1 << bucket ; let index = id - (bucket_size - 1) ; Self { bucket , index } } # [doc = " The size of the bucket this thread's local storage will be in."] pub (crate) fn bucket_size (& self) -> usize { 1 << self . bucket } # [doc = " The thread ID obtained from the thread ID manager."] pub (crate) fn id (& self) -> usize { self . index + (self . bucket_size () - 1) } }
};
}
