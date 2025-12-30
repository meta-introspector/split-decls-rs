// Generated macro for impl_63 (impl)
macro_rules! Depcrate_processingimpl_63 {
() => {
// Module: crate::processing
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'a , 'b > CheckerData < 'a , 'b > { # [doc = " User variable corresponding to proof variable."] # [doc = ""] # [doc = " Returns `None` if the proof variable is an internal or hidden variable."] pub fn user_from_proof_var (self , proof_var : Var) -> Option < Var > { let variables = self . 0 . part (VariablesP) ; variables . var_data . get (proof_var . index ()) . and_then (| var_data | { var_data . user_var . or_else (| | { if var_data . sampling_mode == SamplingMode :: Sample { Some (proof_var) } else { None } }) }) } }
};
}
