// Generated macro for HashResult (type)
macro_rules! Depcrate_query_configHashResult {
() => {
// Module: crate::query::config
// Provides: {"HashResult"}
// Dependencies: {}
pub type HashResult < V > = Option < fn (& mut StableHashingContext < '_ > , & V) -> Fingerprint > ;
};
}
