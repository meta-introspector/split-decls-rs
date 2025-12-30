// Generated macro for impl_111 (impl)
macro_rules! Depcrate_map_kvimpl_111 {
() => {
// Module: crate::map::kv
// Provides: {"impl_111"}
// Dependencies: {}
impl < 'a , T > ZeroMapKV < 'a > for Option < T > where Option < T > : AsULE + 'static , { type Container = ZeroVec < 'a , Option < T > > ; type Slice = ZeroSlice < Option < T > > ; type GetType = < Option < T > as AsULE > :: ULE ; type OwnedType = Option < T > ; }
};
}
