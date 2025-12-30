// Generated macro for impl_125 (impl)
macro_rules! Depcrate_is_serialize_strimpl_125 {
() => {
// Module: crate::is_serialize_str
// Provides: {"impl_125"}
// Dependencies: {}
impl Debug for Unexpected { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { match self { Unexpected :: Str (s) => Debug :: fmt (s , formatter) , Unexpected :: NonStr => formatter . write_str ("non-string") , } } }
};
}
