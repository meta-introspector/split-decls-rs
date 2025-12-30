// Generated macro for detect_target (function)
macro_rules! Depcrate_utildetect_target {
() => {
// Module: crate::util
// Provides: {"detect_target"}
// Dependencies: {}
pub (crate) fn detect_target (sh : & Shell) -> String { match std :: env :: var ("RA_TARGET") { Ok (target) => target , _ => match cmd ! (sh , "rustc --print=host-tuple") . read () { Ok (target) => target , Err (e) => panic ! ("Failed to detect target: {e}\nPlease set RA_TARGET explicitly") , } , } }
};
}
