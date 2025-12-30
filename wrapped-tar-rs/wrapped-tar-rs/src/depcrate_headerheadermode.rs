// Generated macro for HeaderMode (enum)
macro_rules! Depcrate_headerHeaderMode {
() => {
// Module: crate::header
// Provides: {"HeaderMode"}
// Dependencies: {}
# [doc = " Declares the information that should be included when filling a Header"] # [doc = " from filesystem metadata."] # [derive (Clone , Copy , PartialEq , Eq , Debug)] # [non_exhaustive] pub enum HeaderMode { # [doc = " All supported metadata, including mod/access times and ownership will"] # [doc = " be included."] Complete , # [doc = " Only metadata that is directly relevant to the identity of a file will"] # [doc = " be included. In particular, ownership and mod/access times are excluded."] Deterministic , }
};
}
