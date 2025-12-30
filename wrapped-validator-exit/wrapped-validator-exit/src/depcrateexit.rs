// Generated macro for Exit (struct)
macro_rules! DepcrateExit {
() => {
// Module: crate
// Provides: {"Exit"}
// Dependencies: {}
# [derive (Default)] pub struct Exit { exited : bool , exits : Vec < Box < dyn FnOnce () + Send + Sync > > , }
};
}
