// Generated macro for AddSignal (trait)
macro_rules! Depcrate_iterator_backendAddSignal {
() => {
// Module: crate::iterator::backend
// Provides: {"AddSignal"}
// Dependencies: {}
# [doc = " An internal trait to hide adding new signals into a Handle behind a dynamic dispatch."] trait AddSignal : Debug + Send + Sync { fn add_signal (self : Arc < Self > , write : Arc < dyn SelfPipeWrite > , signal : c_int ,) -> Result < SigId , Error > ; }
};
}
