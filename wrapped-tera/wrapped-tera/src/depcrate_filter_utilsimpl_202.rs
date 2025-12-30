// Generated macro for impl_202 (impl)
macro_rules! Depcrate_filter_utilsimpl_202 {
() => {
// Module: crate::filter_utils
// Provides: {"impl_202"}
// Dependencies: {}
impl < K : GetValue > SortStrategy for SortPairs < K > { fn try_add_pair (& mut self , val : & Value , key : & Value) -> Result < () > { SortPairs :: try_add_pair (self , val , key) } fn sort (& mut self) -> Vec < Value > { SortPairs :: sort (self) } }
};
}
