// Generated macro for impl_106 (impl)
macro_rules! Depcrate_yamlimpl_106 {
() => {
// Module: crate::yaml
// Provides: {"impl_106"}
// Dependencies: {}
impl < 'a > Index < & 'a str > for Yaml { type Output = Yaml ; # [doc = " Perform indexing if `self` is a mapping."] # [doc = ""] # [doc = " # Return"] # [doc = " If `self` is a [`Yaml::Hash`], returns an immutable borrow to the value associated to the"] # [doc = " given key in the hash."] # [doc = ""] # [doc = " This function returns a [`Yaml::BadValue`] if the underlying [`type@Hash`] does not contain"] # [doc = " [`Yaml::String`]`{idx}` as a key."] # [doc = ""] # [doc = " This function also returns a [`Yaml::BadValue`] if `self` is not a [`Yaml::Hash`]."] fn index (& self , idx : & 'a str) -> & Yaml { let key = Yaml :: String (idx . to_owned ()) ; match self . as_hash () { Some (h) => h . get (& key) . unwrap_or (& BAD_VALUE) , None => & BAD_VALUE , } } }
};
}
