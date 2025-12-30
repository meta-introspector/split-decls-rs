// Generated macro for impl_35 (impl)
macro_rules! Depcrate_wrap_algorithmsimpl_35 {
() => {
// Module: crate::wrap_algorithms
// Provides: {"impl_35"}
// Dependencies: {}
impl PartialEq for WrapAlgorithm { # [doc = " Compare two wrap algorithms."] # [doc = ""] # [doc = " ```"] # [doc = " use textwrap::WrapAlgorithm;"] # [doc = ""] # [doc = " assert_eq!(WrapAlgorithm::FirstFit, WrapAlgorithm::FirstFit);"] # [doc = " #[cfg(feature = \"smawk\")] {"] # [doc = "     assert_eq!(WrapAlgorithm::new_optimal_fit(), WrapAlgorithm::new_optimal_fit());"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Note that `WrapAlgorithm::Custom` values never compare equal:"] # [doc = ""] # [doc = " ```"] # [doc = " use textwrap::WrapAlgorithm;"] # [doc = ""] # [doc = " assert_ne!(WrapAlgorithm::Custom(|words, line_widths| vec![words]),"] # [doc = "            WrapAlgorithm::Custom(|words, line_widths| vec![words]));"] # [doc = " ```"] fn eq (& self , other : & Self) -> bool { match (self , other) { (WrapAlgorithm :: FirstFit , WrapAlgorithm :: FirstFit) => true , # [cfg (feature = "smawk")] (WrapAlgorithm :: OptimalFit (a) , WrapAlgorithm :: OptimalFit (b)) => a == b , (_ , _) => false , } } }
};
}
