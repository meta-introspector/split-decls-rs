// Generated macro for AutoThinVec (struct)
macro_rules! DepcrateAutoThinVec {
() => {
// Module: crate
// Provides: {"AutoThinVec"}
// Dependencies: {}
# [doc (hidden)] # [cfg (feature = "gecko-ffi")] # [repr (C)] pub struct AutoThinVec < T , const N : usize > { inner : ThinVec < T > , buffer : AutoBuffer < T , N > , _pinned : std :: marker :: PhantomPinned , }
};
}
