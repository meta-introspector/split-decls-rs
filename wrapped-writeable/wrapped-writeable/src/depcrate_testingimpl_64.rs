// Generated macro for impl_64 (impl)
macro_rules! Depcrate_testingimpl_64 {
() => {
// Module: crate::testing
// Provides: {"impl_64"}
// Dependencies: {}
impl PartsWrite for TestWriter { type SubPartsWrite = Self ; fn with_part (& mut self , part : Part , mut f : impl FnMut (& mut Self :: SubPartsWrite) -> fmt :: Result ,) -> fmt :: Result { let start = self . string . len () ; f (self) ? ; let end = self . string . len () ; if start < end { self . parts . push ((start , end , part)) ; } Ok (()) } }
};
}
