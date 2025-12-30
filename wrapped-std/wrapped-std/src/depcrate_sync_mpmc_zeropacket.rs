// Generated macro for Packet (struct)
macro_rules! Depcrate_sync_mpmc_zeroPacket {
() => {
// Module: crate::sync::mpmc::zero
// Provides: {"Packet"}
// Dependencies: {}
# [doc = " A slot for passing one message from a sender to a receiver."] struct Packet < T > { # [doc = " Equals `true` if the packet is allocated on the stack."] on_stack : bool , # [doc = " Equals `true` once the packet is ready for reading or writing."] ready : Atomic < bool > , # [doc = " The message."] msg : UnsafeCell < Option < T > > , }
};
}
