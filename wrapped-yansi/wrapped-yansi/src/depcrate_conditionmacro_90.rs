// Generated macro for macro_90 (macro)
macro_rules! Depcrate_conditionmacro_90 {
() => {
// Module: crate::condition
// Provides: {"macro_90"}
// Dependencies: {}
conditions ! { all (feature = "detect-env" , feature = "detect-tty") Condition :: stdouterr_are_tty () && Condition :: clicolor () && Condition :: no_color () , TTY_AND_COLOR : tty_and_color , TTY_AND_COLOR_LIVE : tty_and_color_live , }
};
}
