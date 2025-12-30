// Generated macro for impl_109 (impl)
macro_rules! Depcrate_yamlimpl_109 {
() => {
// Module: crate::yaml
// Provides: {"impl_109"}
// Dependencies: {}
impl IndexMut < usize > for Yaml { # [doc = " Perform indexing if `self` is a sequence or a mapping."] # [doc = ""] # [doc = " Since we cannot return a mutable borrow to a static [`Yaml::BadValue`] as we return an"] # [doc = " immutable one in [`Index<usize>`], this function panics on out of bounds."] # [doc = ""] # [doc = " # Panics"] # [doc = " This function panics if the index given is out of range (as per [`IndexMut`]). If `self` is"] # [doc = " a [`Yaml::Array`], this is when the index is bigger or equal to the length of the"] # [doc = " underlying `Vec`. If `self` is a [`Yaml::Hash`], this is when the mapping sequence does not"] # [doc = " contain [`Yaml::Integer`]`(idx)` as a key."] # [doc = ""] # [doc = " This function also panics if `self` is not a [`Yaml::Array`] nor a [`Yaml::Hash`]."] fn index_mut (& mut self , idx : usize) -> & mut Yaml { match self { Yaml :: Array (sequence) => sequence . index_mut (idx) , Yaml :: Hash (mapping) => { let key = Yaml :: Integer (i64 :: try_from (idx) . unwrap ()) ; mapping . get_mut (& key) . unwrap () } _ => panic ! ("Attempting to index but `self` is not a sequence nor a mapping") , } } }
};
}
