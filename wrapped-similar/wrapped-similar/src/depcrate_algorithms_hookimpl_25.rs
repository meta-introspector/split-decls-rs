// Generated macro for impl_25 (impl)
macro_rules! Depcrate_algorithms_hookimpl_25 {
() => {
// Module: crate::algorithms::hook
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'a , D : DiffHook + 'a > DiffHook for & 'a mut D { type Error = D :: Error ; # [inline (always)] fn equal (& mut self , old_index : usize , new_index : usize , len : usize) -> Result < () , Self :: Error > { (* self) . equal (old_index , new_index , len) } # [inline (always)] fn delete (& mut self , old_index : usize , old_len : usize , new_index : usize ,) -> Result < () , Self :: Error > { (* self) . delete (old_index , old_len , new_index) } # [inline (always)] fn insert (& mut self , old_index : usize , new_index : usize , new_len : usize ,) -> Result < () , Self :: Error > { (* self) . insert (old_index , new_index , new_len) } # [inline (always)] fn replace (& mut self , old : usize , old_len : usize , new : usize , new_len : usize ,) -> Result < () , Self :: Error > { (* self) . replace (old , old_len , new , new_len) } # [inline (always)] fn finish (& mut self) -> Result < () , Self :: Error > { (* self) . finish () } }
};
}
