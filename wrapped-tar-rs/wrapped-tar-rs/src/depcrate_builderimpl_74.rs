// Generated macro for impl_74 (impl)
macro_rules! Depcrate_builderimpl_74 {
() => {
// Module: crate::builder
// Provides: {"impl_74"}
// Dependencies: {}
impl SparseEntries { fn size (& self) -> u64 { self . entries . last () . map_or (0 , | e | e . offset + e . num_bytes) } }
};
}
