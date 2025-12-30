// Generated macro for impl_23 (impl)
macro_rules! Depcrate_cowimpl_23 {
() => {
// Module: crate::cow
// Provides: {"impl_23"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a , V : VarULE + ? Sized > From < Box < V > > for VarZeroCow < 'a , V > { fn from (other : Box < V >) -> Self { Self :: new_owned (other) } }
};
}
