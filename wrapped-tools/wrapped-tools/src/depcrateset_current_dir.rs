// Generated macro for set_current_dir (function)
macro_rules! Depcrateset_current_dir {
() => {
// Module: crate
// Provides: {"set_current_dir"}
// Dependencies: {}
# [doc = " Set the current working dir to `new_cwd` and return a type that returns to the previous working dir on drop."] pub fn set_current_dir (new_cwd : impl AsRef < Path >) -> std :: io :: Result < AutoRevertToPreviousCWD > { let cwd = env :: current_dir () ? ; env :: set_current_dir (new_cwd) ? ; Ok (AutoRevertToPreviousCWD (cwd)) }
};
}
