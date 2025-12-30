// Generated macro for Case (struct)
macro_rules! Depcrate_runnerCase {
() => {
// Module: crate::runner
// Provides: {"Case"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct Case { pub (crate) path : std :: path :: PathBuf , pub (crate) expected : Option < crate :: schema :: CommandStatus > , pub (crate) timeout : Option < std :: time :: Duration > , pub (crate) default_bin : Option < crate :: schema :: Bin > , pub (crate) env : crate :: schema :: Env , pub (crate) error : Option < SpawnStatus > , }
};
}
