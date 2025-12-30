// Generated macro for impl_116 (impl)
macro_rules! Depcrate_map_kvimpl_116 {
() => {
// Module: crate::map::kv
// Provides: {"impl_116"}
// Dependencies: {}
impl < 'a , T , const N : usize > ZeroMapKV < 'a > for [T ; N] where T : AsULE + 'static , { type Container = ZeroVec < 'a , [T ; N] > ; type Slice = ZeroSlice < [T ; N] > ; type GetType = [T :: ULE ; N] ; type OwnedType = [T ; N] ; }
};
}
