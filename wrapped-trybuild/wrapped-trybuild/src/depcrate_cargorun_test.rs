// Generated macro for run_test (function)
macro_rules! Depcrate_cargorun_test {
() => {
// Module: crate::cargo
// Provides: {"run_test"}
// Dependencies: {}
pub (crate) fn run_test (project : & Project , name : & Name) -> Result < Output > { cargo (project) . arg ("run") . args (target ()) . arg ("--bin") . arg (name) . args (features (project)) . arg ("--quiet") . arg ("--color=never") . output () . map_err (Error :: Cargo) }
};
}
