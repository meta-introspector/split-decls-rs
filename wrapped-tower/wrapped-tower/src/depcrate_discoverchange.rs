// Generated macro for Change (enum)
macro_rules! Depcrate_discoverChange {
() => {
// Module: crate::discover
// Provides: {"Change"}
// Dependencies: {}
# [doc = " A change in the service set."] # [derive (Debug , Clone)] pub enum Change < K , V > { # [doc = " A new service identified by key `K` was identified."] Insert (K , V) , # [doc = " The service identified by key `K` disappeared."] Remove (K) , }
};
}
