// Generated macro for build_test (function)
macro_rules! Depcrate_cargobuild_test {
() => {
// Module: crate::cargo
// Provides: {"build_test"}
// Dependencies: {}
pub (crate) fn build_test (project : & Project , name : & Name) -> Result < Output > { let _ = cargo (project) . arg ("clean") . arg ("--package") . arg (& project . name) . arg ("--color=never") . stdout (Stdio :: null ()) . stderr (Stdio :: null ()) . status () ; cargo (project) . arg (if project . has_pass { "build" } else { "check" }) . args (target ()) . arg ("--bin") . arg (name) . args (features (project)) . arg ("--quiet") . arg ("--color=never") . arg ("--message-format=json") . output () . map_err (Error :: Cargo) }
};
}
