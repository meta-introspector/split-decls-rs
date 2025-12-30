// Generated macro for macro_19 (macro)
macro_rules! Depcratemacro_19 {
() => {
// Module: crate
// Provides: {"macro_19"}
// Dependencies: {}
# [cfg (target_os = "nto")] cfg_select ! { target_env = "nto70" => { # [link (name = "gcc")] unsafe extern "C" { } } _ => { # [link (name = "gcc_s")] unsafe extern "C" { } } }
};
}
