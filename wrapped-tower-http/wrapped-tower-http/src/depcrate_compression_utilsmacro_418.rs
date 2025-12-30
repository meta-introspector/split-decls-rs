// Generated macro for macro_418 (macro)
macro_rules! Depcrate_compression_utilsmacro_418 {
() => {
// Module: crate::compression_utils
// Provides: {"macro_418"}
// Dependencies: {}
pin_project ! { pub (crate) struct BodyIntoStream < B > where B : Body , { # [pin] body : B , yielded_all_data : bool , non_data_frame : Option < Frame < B :: Data >>, } }
};
}
