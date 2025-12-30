// Generated macro for impl_3303 (impl)
macro_rules! Depcrate_sync_poisonimpl_3303 {
() => {
// Module: crate::sync::poison
// Provides: {"impl_3303"}
// Dependencies: {}
impl Flag { # [inline] pub const fn new () -> Flag { Flag { # [cfg (panic = "unwind")] failed : AtomicBool :: new (false) , } } # [doc = " Checks the flag for an unguarded borrow, where we only care about existing poison."] # [inline] pub fn borrow (& self) -> LockResult < () > { if self . get () { Err (PoisonError :: new (())) } else { Ok (()) } } # [doc = " Checks the flag for a guarded borrow, where we may also set poison when `done`."] # [inline] pub fn guard (& self) -> LockResult < Guard > { let ret = Guard { # [cfg (panic = "unwind")] panicking : thread :: panicking () , } ; if self . get () { Err (PoisonError :: new (ret)) } else { Ok (ret) } } # [inline] # [cfg (panic = "unwind")] pub fn done (& self , guard : & Guard) { if ! guard . panicking && thread :: panicking () { self . failed . store (true , Ordering :: Relaxed) ; } } # [inline] # [cfg (not (panic = "unwind"))] pub fn done (& self , _guard : & Guard) { } # [inline] # [cfg (panic = "unwind")] pub fn get (& self) -> bool { self . failed . load (Ordering :: Relaxed) } # [inline (always)] # [cfg (not (panic = "unwind"))] pub fn get (& self) -> bool { false } # [inline] pub fn clear (& self) { # [cfg (panic = "unwind")] self . failed . store (false , Ordering :: Relaxed) } }
};
}
