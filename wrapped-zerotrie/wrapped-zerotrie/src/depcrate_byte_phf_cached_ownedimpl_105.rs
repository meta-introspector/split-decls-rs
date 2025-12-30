// Generated macro for impl_105 (impl)
macro_rules! Depcrate_byte_phf_cached_ownedimpl_105 {
() => {
// Module: crate::byte_phf::cached_owned
// Provides: {"impl_105"}
// Dependencies: {}
impl PerfectByteHashMapCacheOwned { # [doc = " Creates a new empty instance."] pub fn new_empty () -> Self { Self { data : BTreeMap :: new () , } } # [doc = " Gets the [`PerfectByteHashMap`] for the given bytes, calculating it if necessary."] pub fn try_get_or_insert (& mut self , keys : Vec < u8 > ,) -> Result < & PerfectByteHashMap < [u8] > , ZeroTrieBuildError > { let mut_phf = match self . data . entry (keys) { Entry :: Vacant (entry) => { let value = PerfectByteHashMap :: try_new (entry . key ()) ? ; entry . insert (value) } Entry :: Occupied (entry) => entry . into_mut () , } ; Ok (mut_phf . as_borrowed ()) } }
};
}
