// Generated macro for impl_19 (impl)
macro_rules! Depcrate_algorithms_compactimpl_19 {
() => {
// Module: crate::algorithms::compact
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'old , 'new , Old , New , D > DiffHook for Compact < 'old , 'new , Old , New , D > where D : DiffHook , Old : Index < usize > + ? Sized + 'old , New : Index < usize > + ? Sized + 'new , New :: Output : PartialEq < Old :: Output > , { type Error = D :: Error ; # [inline (always)] fn equal (& mut self , old_index : usize , new_index : usize , len : usize) -> Result < () , Self :: Error > { self . ops . push (DiffOp :: Equal { old_index , new_index , len , }) ; Ok (()) } # [inline (always)] fn delete (& mut self , old_index : usize , old_len : usize , new_index : usize ,) -> Result < () , Self :: Error > { self . ops . push (DiffOp :: Delete { old_index , old_len , new_index , }) ; Ok (()) } # [inline (always)] fn insert (& mut self , old_index : usize , new_index : usize , new_len : usize ,) -> Result < () , Self :: Error > { self . ops . push (DiffOp :: Insert { old_index , new_index , new_len , }) ; Ok (()) } fn finish (& mut self) -> Result < () , Self :: Error > { cleanup_diff_ops (self . old , self . new , & mut self . ops) ; for op in & self . ops { op . apply_to_hook (& mut self . d) ? ; } self . d . finish () } }
};
}
