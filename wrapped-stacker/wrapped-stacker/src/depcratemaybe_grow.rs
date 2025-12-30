// Generated macro for maybe_grow (function)
macro_rules! Depcratemaybe_grow {
() => {
// Module: crate
// Provides: {"maybe_grow"}
// Dependencies: {}
# [doc = " Grows the call stack if necessary."] # [doc = ""] # [doc = " This function is intended to be called at manually instrumented points in a program where"] # [doc = " recursion is known to happen quite a bit. This function will check to see if we're within"] # [doc = " `red_zone` bytes of the end of the stack, and if so it will allocate a new stack of at least"] # [doc = " `stack_size` bytes."] # [doc = ""] # [doc = " The closure `f` is guaranteed to run on a stack with at least `red_zone` bytes, and it will be"] # [doc = " run on the current stack if there's space available."] # [inline (always)] pub fn maybe_grow < R , F : FnOnce () -> R > (red_zone : usize , stack_size : usize , callback : F) -> R { let enough_space = match remaining_stack () { Some (remaining) => remaining >= red_zone , None => false , } ; if enough_space { callback () } else { grow (stack_size , callback) } }
};
}
