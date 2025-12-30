// Generated macro for impl_180 (impl)
macro_rules! Depcrate_featuresimpl_180 {
() => {
// Module: crate::features
// Provides: {"impl_180"}
// Dependencies: {}
impl fmt :: Display for Status { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let as_str = match * self { Status :: Accepted => "accepted" , Status :: Unstable => "unstable" , Status :: Removed => "removed" , } ; fmt :: Display :: fmt (as_str , f) } }
};
}
