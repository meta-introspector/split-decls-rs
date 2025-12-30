// Generated macro for impl_167 (impl)
macro_rules! Depcrate_serimpl_167 {
() => {
// Module: crate::ser
// Provides: {"impl_167"}
// Dependencies: {}
impl < M > SerializeSeqAsMapValue < M > { pub fn new (map : M , len : Option < usize >) -> Self { SerializeSeqAsMapValue { map , fields : Vec :: with_capacity (len . unwrap_or (0)) , } } }
};
}
