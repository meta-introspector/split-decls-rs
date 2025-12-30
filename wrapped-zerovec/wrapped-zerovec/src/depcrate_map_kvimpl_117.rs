// Generated macro for impl_117 (impl)
macro_rules! Depcrate_map_kvimpl_117 {
() => {
// Module: crate::map::kv
// Provides: {"impl_117"}
// Dependencies: {}
impl < 'a , T > ZeroMapKV < 'a > for ZeroSlice < T > where T : AsULE + 'static , { type Container = VarZeroVec < 'a , ZeroSlice < T > > ; type Slice = VarZeroSlice < ZeroSlice < T > > ; type GetType = ZeroSlice < T > ; type OwnedType = Box < ZeroSlice < T > > ; }
};
}
