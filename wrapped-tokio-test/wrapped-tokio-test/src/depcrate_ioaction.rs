// Generated macro for Action (enum)
macro_rules! Depcrate_ioAction {
() => {
// Module: crate::io
// Provides: {"Action"}
// Dependencies: {}
# [derive (Debug , Clone)] enum Action { Read (Vec < u8 >) , Write (Vec < u8 >) , Wait (Duration) , ReadError (Option < Arc < io :: Error > >) , WriteError (Option < Arc < io :: Error > >) , }
};
}
