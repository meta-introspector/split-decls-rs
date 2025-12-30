// Generated macro for impl_23 (impl)
macro_rules! Depcrate_flagsimpl_23 {
() => {
// Module: crate::flags
// Provides: {"impl_23"}
// Dependencies: {}
impl FromStr for MeasurementType { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "build" => Ok (Self :: Build) , "rustc_tests" => Ok (Self :: RustcTests) , "self" => Ok (Self :: AnalyzeSelf) , "ripgrep-13.0.0" => Ok (Self :: AnalyzeRipgrep) , "webrender-2022" => Ok (Self :: AnalyzeWebRender) , "diesel-1.4.8" => Ok (Self :: AnalyzeDiesel) , "hyper-0.14.18" => Ok (Self :: AnalyzeHyper) , _ => Err ("Invalid option" . to_owned ()) , } } }
};
}
