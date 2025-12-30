// Generated macro for ptr_vec_to_ref_vec (function)
macro_rules! Depcrate_utilsptr_vec_to_ref_vec {
() => {
// Module: crate::utils
// Provides: {"ptr_vec_to_ref_vec"}
// Dependencies: {}
# [inline] pub (crate) fn ptr_vec_to_ref_vec < T > (vec : & [ptr :: P < T >]) -> Vec < & T > { vec . iter () . map (| x | & * * x) . collect :: < Vec < _ > > () }
};
}
