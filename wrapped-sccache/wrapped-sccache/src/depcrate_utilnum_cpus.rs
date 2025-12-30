// Generated macro for num_cpus (function)
macro_rules! Depcrate_utilnum_cpus {
() => {
// Module: crate::util
// Provides: {"num_cpus"}
// Dependencies: {}
pub fn num_cpus () -> usize { std :: thread :: available_parallelism () . map_or (1 , std :: num :: NonZeroUsize :: get) }
};
}
