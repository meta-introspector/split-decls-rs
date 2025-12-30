// Generated macro for remaining_stack (function)
macro_rules! Depcrateremaining_stack {
() => {
// Module: crate
// Provides: {"remaining_stack"}
// Dependencies: {}
# [doc = " Queries the amount of remaining stack as interpreted by this library."] # [doc = ""] # [doc = " This function will return the amount of stack space left which will be used"] # [doc = " to determine whether a stack switch should be made or not."] pub fn remaining_stack () -> Option < usize > { let current_ptr = current_stack_ptr () ; get_stack_limit () . map (| limit | current_ptr . saturating_sub (limit)) }
};
}
