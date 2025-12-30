// Generated macro for impl_496 (impl)
macro_rules! Depcrate_zerovec_serdeimpl_496 {
() => {
// Module: crate::zerovec::serde
// Provides: {"impl_496"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `serde` Cargo feature of the `zerovec` crate"] # [cfg (feature = "alloc")] impl < 'de , T > Deserialize < 'de > for alloc :: boxed :: Box < ZeroSlice < T > > where T : Deserialize < 'de > + AsULE + 'static , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let mut zv = ZeroVec :: < T > :: deserialize (deserializer) ? ; let vec = zv . with_mut (core :: mem :: take) ; Ok (ZeroSlice :: from_boxed_slice (vec . into_boxed_slice ())) } }
};
}
