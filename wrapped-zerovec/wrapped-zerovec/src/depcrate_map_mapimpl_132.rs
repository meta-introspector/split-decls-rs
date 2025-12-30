// Generated macro for impl_132 (impl)
macro_rules! Depcrate_map_mapimpl_132 {
() => {
// Module: crate::map::map
// Provides: {"impl_132"}
// Dependencies: {}
impl < 'a , K , V > ZeroMap < 'a , K , V > where K : AsULE + ZeroMapKV < 'a , Container = ZeroVec < 'a , K > > , V : ZeroMapKV < 'a > + ? Sized , { # [doc = " Cast a `ZeroMap<K, V>` to `ZeroMap<P, V>` where `K` and `P` are [`AsULE`] types"] # [doc = " with the same representation."] # [doc = ""] # [doc = " # Unchecked Invariants"] # [doc = ""] # [doc = " If `K` and `P` have different ordering semantics, unexpected behavior may occur."] pub fn cast_zv_k_unchecked < P > (self) -> ZeroMap < 'a , P , V > where P : AsULE < ULE = K :: ULE > + ZeroMapKV < 'a , Container = ZeroVec < 'a , P > > , { ZeroMap { keys : self . keys . cast () , values : self . values , } } # [doc = " Convert a `ZeroMap<K, V>` to `ZeroMap<P, V>` where `K` and `P` are [`AsULE`] types"] # [doc = " with the same size."] # [doc = ""] # [doc = " # Unchecked Invariants"] # [doc = ""] # [doc = " If `K` and `P` have different ordering semantics, unexpected behavior may occur."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `K::ULE` and `P::ULE` are not the same size."] pub fn try_convert_zv_k_unchecked < P > (self) -> Result < ZeroMap < 'a , P , V > , UleError > where P : AsULE + ZeroMapKV < 'a , Container = ZeroVec < 'a , P > > , { Ok (ZeroMap { keys : self . keys . try_into_converted () ? , values : self . values , }) } }
};
}
