// Generated macro for impl_880 (impl)
macro_rules! Depcrate_punctuatedimpl_880 {
() => {
// Module: crate::punctuated
// Provides: {"impl_880"}
// Dependencies: {}
# [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl < T : Debug , P : Debug > Debug for Punctuated < T , P > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut list = f . debug_list () ; for (t , p) in & self . inner { list . entry (t) ; list . entry (p) ; } if let Some (last) = & self . last { list . entry (last) ; } list . finish () } }
};
}
