// Generated macro for impl_60 (impl)
macro_rules! Depcrate_libs_mini_mokaimpl_60 {
() => {
// Module: crate::libs::mini_moka
// Provides: {"impl_60"}
// Dependencies: {}
# [cfg (feature = "dashmap")] impl < K : Eq + Hash , V , S : BuildHasher + Clone > crate :: map :: EntryRef < K , V > for mini_moka :: sync :: EntryRef < '_ , K , V , S > { fn get_ref (& self) -> (& K , & V) { self . pair () } }
};
}
