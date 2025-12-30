// Generated macro for impl_200 (impl)
macro_rules! Depcrate_filter_utilsimpl_200 {
() => {
// Module: crate::filter_utils
// Provides: {"impl_200"}
// Dependencies: {}
impl < K : GetValue > SortPairs < K > { fn try_add_pair (& mut self , val : & Value , key : & Value) -> Result < () > { let key = K :: get_value (key) ? ; self . pairs . push ((val . clone () , key)) ; Ok (()) } fn sort (& mut self) -> Vec < Value > { self . pairs . sort_by_key (| a | a . 1 . clone ()) ; self . pairs . iter () . map (| a | a . 0 . clone ()) . collect () } }
};
}
