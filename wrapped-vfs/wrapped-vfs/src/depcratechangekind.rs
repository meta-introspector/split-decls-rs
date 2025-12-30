// Generated macro for ChangeKind (enum)
macro_rules! DepcrateChangeKind {
() => {
// Module: crate
// Provides: {"ChangeKind"}
// Dependencies: {}
# [doc = " Kind of [file change](ChangedFile)."] # [derive (Eq , PartialEq , Debug)] pub enum ChangeKind { # [doc = " The file was (re-)created"] Create , # [doc = " The file was modified"] Modify , # [doc = " The file was deleted"] Delete , }
};
}
