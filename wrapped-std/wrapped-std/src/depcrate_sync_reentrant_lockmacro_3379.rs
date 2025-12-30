// Generated macro for macro_3379 (macro)
macro_rules! Depcrate_sync_reentrant_lockmacro_3379 {
() => {
// Module: crate::sync::reentrant_lock
// Provides: {"macro_3379"}
// Dependencies: {}
cfg_select ! (target_has_atomic = "64" => { use crate :: sync :: atomic :: { Atomic , AtomicU64 , Ordering :: Relaxed } ; struct Tid (Atomic < u64 >) ; impl Tid { const fn new () -> Self { Self (AtomicU64 :: new (0)) } # [inline] fn contains (& self , owner : ThreadId) -> bool { owner . as_u64 () . get () == self . 0 . load (Relaxed) } # [inline] unsafe fn set (& self , tid : Option < ThreadId >) { let value = tid . map_or (0 , | tid | tid . as_u64 () . get ()) ; self . 0 . store (value , Relaxed) ; } } } _ => { # [doc = " Returns the address of a TLS variable. This is guaranteed to"] # [doc = " be unique across all currently alive threads."] fn tls_addr () -> usize { thread_local ! { static X : u8 = const { 0u8 } } ; X . with (| p | <* const u8 >:: addr (p)) } use crate :: sync :: atomic :: { Atomic , AtomicUsize , Ordering , } ; struct Tid { tls_addr : Atomic < usize >, tid : UnsafeCell < u64 >, } unsafe impl Send for Tid { } unsafe impl Sync for Tid { } impl Tid { const fn new () -> Self { Self { tls_addr : AtomicUsize :: new (0) , tid : UnsafeCell :: new (0) } } # [inline] fn contains (& self , owner : ThreadId) -> bool { let tls_addr = tls_addr () ; self . tls_addr . load (Ordering :: Relaxed) == tls_addr && unsafe { * self . tid . get () } == owner . as_u64 () . get () } # [inline] unsafe fn set (& self , tid : Option < ThreadId >) { let tls_addr = if tid . is_some () { tls_addr () } else { 0 } ; let value = tid . map_or (0 , | tid | tid . as_u64 () . get ()) ; self . tls_addr . store (tls_addr , Ordering :: Relaxed) ; unsafe { * self . tid . get () = value } ; } } }) ;
};
}
