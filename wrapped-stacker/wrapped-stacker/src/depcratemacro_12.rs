// Generated macro for macro_12 (macro)
macro_rules! Depcratemacro_12 {
() => {
// Module: crate
// Provides: {"macro_12"}
// Dependencies: {}
thread_local ! { static STACK_LIMIT : Cell < Option < usize >> = Cell :: new (unsafe { backends :: guess_os_stack_limit () }) }
};
}
