// Generated macro for impl_107 (impl)
macro_rules! Depcrate_diagnostics_utilsimpl_107 {
() => {
// Module: crate::diagnostics::utils
// Provides: {"impl_107"}
// Dependencies: {}
impl FromStr for Applicability { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "machine-applicable" => Ok (Applicability :: MachineApplicable) , "maybe-incorrect" => Ok (Applicability :: MaybeIncorrect) , "has-placeholders" => Ok (Applicability :: HasPlaceholders) , "unspecified" => Ok (Applicability :: Unspecified) , _ => Err (()) , } } }
};
}
