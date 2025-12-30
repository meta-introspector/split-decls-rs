// Generated macro for macro_11 (macro)
macro_rules! Depcratemacro_11 {
() => {
// Module: crate
// Provides: {"macro_11"}
// Dependencies: {}
psm_stack_information ! (yes { fn current_stack_ptr () -> usize { psm :: stack_pointer () as usize } } no { # [inline (always)] fn current_stack_ptr () -> usize { unsafe { let mut x = std :: mem :: MaybeUninit ::< u8 >:: uninit () ; x . as_mut_ptr () . write_volatile (42) ; x . as_ptr () as usize } } }) ;
};
}
