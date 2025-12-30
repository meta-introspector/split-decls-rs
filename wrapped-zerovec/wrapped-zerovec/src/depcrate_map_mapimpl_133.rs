// Generated macro for impl_133 (impl)
macro_rules! Depcrate_map_mapimpl_133 {
() => {
// Module: crate::map::map
// Provides: {"impl_133"}
// Dependencies: {}
impl < 'a , K , V > ZeroMap < 'a , K , V > where K : ZeroMapKV < 'a > + ? Sized , V : AsULE + ZeroMapKV < 'a , Container = ZeroVec < 'a , V > > , { # [doc = " Cast a `ZeroMap<K, V>` to `ZeroMap<K, P>` where `V` and `P` are [`AsULE`] types"] # [doc = " with the same representation."] # [doc = ""] # [doc = " # Unchecked Invariants"] # [doc = ""] # [doc = " If `V` and `P` have different ordering semantics, unexpected behavior may occur."] pub fn cast_zv_v_unchecked < P > (self) -> ZeroMap < 'a , K , P > where P : AsULE < ULE = V :: ULE > + ZeroMapKV < 'a , Container = ZeroVec < 'a , P > > , { ZeroMap { keys : self . keys , values : self . values . cast () , } } # [doc = " Convert a `ZeroMap<K, V>` to `ZeroMap<K, P>` where `V` and `P` are [`AsULE`] types"] # [doc = " with the same size."] # [doc = ""] # [doc = " # Unchecked Invariants"] # [doc = ""] # [doc = " If `V` and `P` have different ordering semantics, unexpected behavior may occur."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `V::ULE` and `P::ULE` are not the same size."] pub fn try_convert_zv_v_unchecked < P > (self) -> Result < ZeroMap < 'a , K , P > , UleError > where P : AsULE + ZeroMapKV < 'a , Container = ZeroVec < 'a , P > > , { Ok (ZeroMap { keys : self . keys , values : self . values . try_into_converted () ? , }) } }
};
}
