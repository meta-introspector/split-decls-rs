// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
impl < R : Unsigned > StreamCipherCore for SalsaCore < R > { # [inline (always)] fn remaining_blocks (& self) -> Option < usize > { let rem = u64 :: MAX - self . get_block_pos () ; rem . try_into () . ok () } fn process_with_backend (& mut self , f : impl StreamCipherClosure < BlockSize = Self :: BlockSize >) { f . call (& mut backends :: soft :: Backend (self)) ; } }
};
}
