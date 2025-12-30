// Generated macro for impl_564 (impl)
macro_rules! Depcrate_msgs_ffdhe_groupsimpl_564 {
() => {
// Module: crate::msgs::ffdhe_groups
// Provides: {"impl_564"}
// Dependencies: {}
impl < 'a > FfdheGroup < 'a > { # [doc = " Construct an `FfdheGroup` from the given `p` and `g`, trimming any potential leading zeros."] pub fn from_params_trimming_leading_zeros (p : & 'a [u8] , g : & 'a [u8]) -> Self { fn trim_leading_zeros (buf : & [u8]) -> & [u8] { for start in 0 .. buf . len () { if buf [start] != 0 { return & buf [start ..] ; } } & [] } FfdheGroup { p : trim_leading_zeros (p) , g : trim_leading_zeros (g) , } } }
};
}
