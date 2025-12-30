// Generated macro for impl_28 (impl)
macro_rules! Depcrate_algorithms_hookimpl_28 {
() => {
// Module: crate::algorithms::hook
// Provides: {"impl_28"}
// Dependencies: {}
impl < D : DiffHook > DiffHook for NoFinishHook < D > { type Error = D :: Error ; # [inline (always)] fn equal (& mut self , old_index : usize , new_index : usize , len : usize) -> Result < () , Self :: Error > { self . 0 . equal (old_index , new_index , len) } # [inline (always)] fn delete (& mut self , old_index : usize , old_len : usize , new_index : usize ,) -> Result < () , Self :: Error > { self . 0 . delete (old_index , old_len , new_index) } # [inline (always)] fn insert (& mut self , old_index : usize , new_index : usize , new_len : usize ,) -> Result < () , Self :: Error > { self . 0 . insert (old_index , new_index , new_len) } # [inline (always)] fn replace (& mut self , old_index : usize , old_len : usize , new_index : usize , new_len : usize ,) -> Result < () , Self :: Error > { self . 0 . replace (old_index , old_len , new_index , new_len) } fn finish (& mut self) -> Result < () , Self :: Error > { Ok (()) } }
};
}
