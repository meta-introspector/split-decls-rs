// Generated macro for impl_8 (impl)
macro_rules! Depcrate_algorithms_captureimpl_8 {
() => {
// Module: crate::algorithms::capture
// Provides: {"impl_8"}
// Dependencies: {}
impl DiffHook for Capture { type Error = Infallible ; # [inline (always)] fn equal (& mut self , old_index : usize , new_index : usize , len : usize) -> Result < () , Self :: Error > { self . 0 . push (DiffOp :: Equal { old_index , new_index , len , }) ; Ok (()) } # [inline (always)] fn delete (& mut self , old_index : usize , old_len : usize , new_index : usize ,) -> Result < () , Self :: Error > { self . 0 . push (DiffOp :: Delete { old_index , old_len , new_index , }) ; Ok (()) } # [inline (always)] fn insert (& mut self , old_index : usize , new_index : usize , new_len : usize ,) -> Result < () , Self :: Error > { self . 0 . push (DiffOp :: Insert { old_index , new_index , new_len , }) ; Ok (()) } # [inline (always)] fn replace (& mut self , old_index : usize , old_len : usize , new_index : usize , new_len : usize ,) -> Result < () , Self :: Error > { self . 0 . push (DiffOp :: Replace { old_index , old_len , new_index , new_len , }) ; Ok (()) } }
};
}
