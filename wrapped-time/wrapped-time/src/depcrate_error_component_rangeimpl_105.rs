// Generated macro for impl_105 (impl)
macro_rules! Depcrate_error_component_rangeimpl_105 {
() => {
// Module: crate::error::component_range
// Provides: {"impl_105"}
// Dependencies: {}
impl hash :: Hash for ComponentRange { # [inline] fn hash < H : hash :: Hasher > (& self , state : & mut H) { self . name . hash (state) ; self . minimum . hash (state) ; self . maximum . hash (state) ; self . value . hash (state) ; self . conditional_message . is_some () . hash (state) ; } }
};
}
