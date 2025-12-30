// Generated macro for impl_108 (impl)
macro_rules! Depcrate_yamlimpl_108 {
() => {
// Module: crate::yaml
// Provides: {"impl_108"}
// Dependencies: {}
impl Index < usize > for Yaml { type Output = Yaml ; # [doc = " Perform indexing if `self` is a sequence or a mapping."] # [doc = ""] # [doc = " # Return"] # [doc = " If `self` is a [`Yaml::Array`], returns an immutable borrow to the value located at the"] # [doc = " given index in the array."] # [doc = ""] # [doc = " Otherwise, if `self` is a [`Yaml::Hash`], returns a borrow to the value whose key is"] # [doc = " [`Yaml::Integer`]`(idx)` (this would not work if the key is [`Yaml::String`]`(\"1\")`."] # [doc = ""] # [doc = " This function returns a [`Yaml::BadValue`] if the index given is out of range. If `self` is"] # [doc = " a [`Yaml::Array`], this is when the index is bigger or equal to the length of the"] # [doc = " underlying `Vec`. If `self` is a [`Yaml::Hash`], this is when the mapping sequence does not"] # [doc = " contain [`Yaml::Integer`]`(idx)` as a key."] # [doc = ""] # [doc = " This function also returns a [`Yaml::BadValue`] if `self` is not a [`Yaml::Array`] nor a"] # [doc = " [`Yaml::Hash`]."] fn index (& self , idx : usize) -> & Yaml { if let Some (v) = self . as_vec () { v . get (idx) . unwrap_or (& BAD_VALUE) } else if let Some (v) = self . as_hash () { let key = Yaml :: Integer (i64 :: try_from (idx) . unwrap ()) ; v . get (& key) . unwrap_or (& BAD_VALUE) } else { & BAD_VALUE } } }
};
}
