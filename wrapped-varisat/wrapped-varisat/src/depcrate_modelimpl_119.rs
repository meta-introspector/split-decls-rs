// Generated macro for impl_119 (impl)
macro_rules! Depcrate_modelimpl_119 {
() => {
// Module: crate::model
// Provides: {"impl_119"}
// Dependencies: {}
impl Model { # [doc = " Assignment of the global model."] # [doc = ""] # [doc = " Only valid if the solver state is SAT."] pub fn assignment (& self) -> & [Option < bool >] { & self . assignment } # [doc = " Whether a given global literal is true in the model assignment."] # [doc = ""] # [doc = " Only valid if the solver state is SAT."] pub fn lit_is_true (& self , global_lit : Lit) -> bool { self . assignment [global_lit . index ()] == Some (global_lit . is_positive ()) } }
};
}
