// Generated macro for impl_638 (impl)
macro_rules! Depcrate_ule_nicheimpl_638 {
() => {
// Module: crate::ule::niche
// Provides: {"impl_638"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a , T : AsULE + 'static , const N : usize > ZeroMapKV < 'a > for NichedOption < T , N > where T :: ULE : NicheBytes < N > , { type Container = ZeroVec < 'a , NichedOption < T , N > > ; type Slice = ZeroSlice < NichedOption < T , N > > ; type GetType = < NichedOption < T , N > as AsULE > :: ULE ; type OwnedType = Self ; }
};
}
