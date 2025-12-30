// Generated macro for impl_138 (impl)
macro_rules! Depcrate_map_mapimpl_138 {
() => {
// Module: crate::map::map
// Provides: {"impl_138"}
// Dependencies: {}
impl < 'a , K , V > From < ZeroMapBorrowed < 'a , K , V > > for ZeroMap < 'a , K , V > where K : ZeroMapKV < 'a > , V : ZeroMapKV < 'a > , K : ? Sized , V : ? Sized , { fn from (other : ZeroMapBorrowed < 'a , K , V >) -> Self { Self { keys : K :: Container :: zvl_from_borrowed (other . keys) , values : V :: Container :: zvl_from_borrowed (other . values) , } } }
};
}
