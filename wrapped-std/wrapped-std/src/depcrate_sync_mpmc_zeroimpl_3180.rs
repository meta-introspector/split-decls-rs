// Generated macro for impl_3180 (impl)
macro_rules! Depcrate_sync_mpmc_zeroimpl_3180 {
() => {
// Module: crate::sync::mpmc::zero
// Provides: {"impl_3180"}
// Dependencies: {}
impl < T > Packet < T > { # [doc = " Creates an empty packet on the stack."] fn empty_on_stack () -> Packet < T > { Packet { on_stack : true , ready : AtomicBool :: new (false) , msg : UnsafeCell :: new (None) } } # [doc = " Creates a packet on the stack, containing a message."] fn message_on_stack (msg : T) -> Packet < T > { Packet { on_stack : true , ready : AtomicBool :: new (false) , msg : UnsafeCell :: new (Some (msg)) } } # [doc = " Waits until the packet becomes ready for reading or writing."] fn wait_ready (& self) { let backoff = Backoff :: new () ; while ! self . ready . load (Ordering :: Acquire) { backoff . spin_heavy () ; } } }
};
}
