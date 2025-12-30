// Generated macro for impl_1149 (impl)
macro_rules! Depcrate_distimpl_1149 {
() => {
// Module: crate::dist
// Provides: {"impl_1149"}
// Dependencies: {}
impl fmt :: Display for JobState { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use self :: JobState :: * ; match * self { Pending => "pending" , Ready => "ready" , Started => "started" , Complete => "complete" , } . fmt (f) } }
};
}
