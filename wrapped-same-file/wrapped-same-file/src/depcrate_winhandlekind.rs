// Generated macro for HandleKind (enum)
macro_rules! Depcrate_winHandleKind {
() => {
// Module: crate::win
// Provides: {"HandleKind"}
// Dependencies: {}
# [derive (Debug)] enum HandleKind { # [doc = " Used when opening a file or acquiring ownership of a file."] Owned (winutil :: Handle) , # [doc = " Used for stdio."] Borrowed (winutil :: HandleRef) , }
};
}
