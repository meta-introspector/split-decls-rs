// Generated macro for grow (function)
macro_rules! Depcrategrow {
() => {
// Module: crate
// Provides: {"grow"}
// Dependencies: {}
# [doc = " Always creates a new stack for the passed closure to run on."] # [doc = " The closure will still be on the same thread as the caller of `grow`."] # [doc = " This will allocate a new stack with at least `stack_size` bytes."] pub fn grow < R , F : FnOnce () -> R > (stack_size : usize , callback : F) -> R { let mut opt_callback = Some (callback) ; let mut ret = None ; let ret_ref = & mut ret ; let dyn_callback : & mut dyn FnMut () = & mut | | { let taken_callback = opt_callback . take () . unwrap () ; * ret_ref = Some (taken_callback ()) ; } ; _grow (stack_size , dyn_callback) ; ret . unwrap () }
};
}
