// Generated macro for macro_15 (macro)
macro_rules! Depcratemacro_15 {
() => {
// Module: crate
// Provides: {"macro_15"}
// Dependencies: {}
psm_stack_manipulation ! { yes { # [cfg (not (any (target_arch = "wasm32" , target_os = "hermit")))] # [path = "mmap_stack_restore_guard.rs"] mod stack_restore_guard ; # [cfg (any (target_arch = "wasm32" , target_os = "hermit"))] # [path = "alloc_stack_restore_guard.rs"] mod stack_restore_guard ; use stack_restore_guard :: StackRestoreGuard ; fn _grow (requested_stack_size : usize , callback : & mut dyn FnMut ()) { unsafe { let guard = StackRestoreGuard :: new (requested_stack_size) ; let (stack_base , allocated_stack_size) = guard . stack_area () ; debug_assert ! (allocated_stack_size >= requested_stack_size) ; set_stack_limit (Some (stack_base as usize)) ; let panic = psm :: on_stack (stack_base , requested_stack_size , move || { std :: panic :: catch_unwind (std :: panic :: AssertUnwindSafe (callback)) . err () }) ; drop (guard) ; if let Some (p) = panic { std :: panic :: resume_unwind (p) ; } } } } no { # [cfg (not (windows))] fn _grow (stack_size : usize , callback : & mut dyn FnMut ()) { let _ = stack_size ; callback () ; } # [cfg (windows)] use backends :: windows :: _grow ; } }
};
}
