// Generated macro for SpooledData (enum)
macro_rules! Depcrate_spooledSpooledData {
() => {
// Module: crate::spooled
// Provides: {"SpooledData"}
// Dependencies: {}
# [doc = " A wrapper for the two states of a [`SpooledTempFile`]. Either:"] # [doc = ""] # [doc = " 1. An in-memory [`Cursor`] representing the state of the file."] # [doc = " 2. A temporary [`File`]."] # [derive (Debug)] pub enum SpooledData { InMemory (Cursor < Vec < u8 > >) , OnDisk (File) , }
};
}
