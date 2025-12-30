// Generated macro for impl_107 (impl)
macro_rules! Depcrate_yamlimpl_107 {
() => {
// Module: crate::yaml
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'a > IndexMut < & 'a str > for Yaml { # [doc = " Perform indexing if `self` is a mapping."] # [doc = ""] # [doc = " Since we cannot return a mutable borrow to a static [`Yaml::BadValue`] as we return an"] # [doc = " immutable one in [`Index<&'a str>`], this function panics on out of bounds."] # [doc = ""] # [doc = " # Panics"] # [doc = " This function panics if the given key is not contained in `self` (as per [`IndexMut`])."] # [doc = ""] # [doc = " This function also panics if `self` is not a [`Yaml::Hash`]."] fn index_mut (& mut self , idx : & 'a str) -> & mut Yaml { let key = Yaml :: String (idx . to_owned ()) ; match self . as_mut_hash () { Some (h) => h . get_mut (& key) . unwrap () , None => panic ! ("Not a hash type") , } } }
};
}
