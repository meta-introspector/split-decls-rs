// Generated macro for impl_36 (impl)
macro_rules! Depcrate_wrap_algorithmsimpl_36 {
() => {
// Module: crate::wrap_algorithms
// Provides: {"impl_36"}
// Dependencies: {}
impl WrapAlgorithm { # [doc = " Create new wrap algorithm."] # [doc = ""] # [doc = " The best wrapping algorithm is used by default, i.e.,"] # [doc = " [`WrapAlgorithm::OptimalFit`] if available, otherwise"] # [doc = " [`WrapAlgorithm::FirstFit`]."] pub const fn new () -> Self { # [cfg (not (feature = "smawk"))] { WrapAlgorithm :: FirstFit } # [cfg (feature = "smawk")] { WrapAlgorithm :: new_optimal_fit () } } # [doc = " New [`WrapAlgorithm::OptimalFit`] with default penalties. This"] # [doc = " works well for monospace text."] # [doc = ""] # [doc = " **Note:** Only available when the `smawk` Cargo feature is"] # [doc = " enabled."] # [cfg (feature = "smawk")] pub const fn new_optimal_fit () -> Self { WrapAlgorithm :: OptimalFit (Penalties :: new ()) } # [doc = " Wrap words according to line widths."] # [doc = ""] # [doc = " The `line_widths` slice gives the target line width for each"] # [doc = " line (the last slice element is repeated as necessary). This"] # [doc = " can be used to implement hanging indentation."] # [inline] pub fn wrap < 'a , 'b > (& self , words : & 'b [Word < 'a >] , line_widths : & 'b [usize] ,) -> Vec < & 'b [Word < 'a >] > { let f64_line_widths = line_widths . iter () . map (| w | * w as f64) . collect :: < Vec < _ > > () ; match self { WrapAlgorithm :: FirstFit => wrap_first_fit (words , & f64_line_widths) , # [cfg (feature = "smawk")] WrapAlgorithm :: OptimalFit (penalties) => { wrap_optimal_fit (words , & f64_line_widths , penalties) . unwrap () } WrapAlgorithm :: Custom (func) => func (words , line_widths) , } } }
};
}
