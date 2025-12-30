// Generated macro for macro_87 (macro)
macro_rules! Depcrate_conditionmacro_87 {
() => {
// Module: crate::condition
// Provides: {"macro_87"}
// Dependencies: {}
conditions ! { feature = "detect-tty" is_tty (& std :: io :: stdout ()) , STDOUT_IS_TTY : stdout_is_tty , STDOUT_IS_TTY_LIVE : stdout_is_tty_live , is_tty (& std :: io :: stderr ()) , STDERR_IS_TTY : stderr_is_tty , STDERR_IS_TTY_LIVE : stderr_is_tty_live , is_tty (& std :: io :: stdin ()) , STDIN_IS_TTY : stdin_is_tty , STDIN_IS_TTY_LIVE : stdin_is_tty_live , is_tty (& std :: io :: stdout ()) && is_tty (& std :: io :: stderr ()) , STDOUTERR_ARE_TTY : stdouterr_are_tty , STDOUTERR_ARE_TTY_LIVE : stdouterr_are_tty_live , }
};
}
