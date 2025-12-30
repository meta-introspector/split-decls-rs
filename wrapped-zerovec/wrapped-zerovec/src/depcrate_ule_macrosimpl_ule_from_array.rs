// Generated macro for impl_ule_from_array (macro)
macro_rules! Depcrate_ule_macrosimpl_ule_from_array {
() => {
// Module: crate::ule::macros
// Provides: {"impl_ule_from_array"}
// Dependencies: {}
# [doc = " Given `Self` (`$aligned`), `Self::ULE` (`$unaligned`), and a conversion function (`$single` or"] # [doc = " `Self::from_aligned`), implement `from_array` for arrays of `$aligned` to `$unaligned`."] # [doc = ""] # [doc = " The `$default` argument is due to current compiler limitations."] # [doc = " Pass any (cheap to construct) value."] # [macro_export] macro_rules ! impl_ule_from_array { ($ aligned : ty , $ unaligned : ty , $ default : expr , $ single : path) => { # [doc = concat ! ("Convert an array of `" , stringify ! ($ aligned) , "` to an array of `" , stringify ! ($ unaligned) , "`.")] pub const fn from_array < const N : usize > (arr : [$ aligned ; N]) -> [Self ; N] { let mut result = [$ default ; N] ; let mut i = 0 ; # [expect (clippy :: indexing_slicing)] while i < N { result [i] = $ single (arr [i]) ; i += 1 ; } result } } ; ($ aligned : ty , $ unaligned : ty , $ default : expr) => { impl_ule_from_array ! ($ aligned , $ unaligned , $ default , Self :: from_aligned) ; } ; }
};
}
