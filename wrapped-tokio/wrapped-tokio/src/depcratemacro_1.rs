// Generated macro for macro_1 (macro)
macro_rules! Depcratemacro_1 {
() => {
// Module: crate
// Provides: {"macro_1"}
// Dependencies: {}
# [cfg (not (any (target_pointer_width = "32" , target_pointer_width = "64")))] compile_error ! { "Tokio requires the platform pointer width to be at least 32 bits" }
};
}
