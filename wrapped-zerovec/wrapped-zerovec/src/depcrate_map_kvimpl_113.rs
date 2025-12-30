// Generated macro for impl_113 (impl)
macro_rules! Depcrate_map_kvimpl_113 {
() => {
// Module: crate::map::kv
// Provides: {"impl_113"}
// Dependencies: {}
impl < 'a , A , B > ZeroMapKV < 'a > for VarTupleULE < A , B > where A : AsULE + 'static , B : VarULE + ? Sized , { type Container = VarZeroVec < 'a , VarTupleULE < A , B > > ; type Slice = VarZeroSlice < VarTupleULE < A , B > > ; type GetType = VarTupleULE < A , B > ; type OwnedType = Box < VarTupleULE < A , B > > ; }
};
}
