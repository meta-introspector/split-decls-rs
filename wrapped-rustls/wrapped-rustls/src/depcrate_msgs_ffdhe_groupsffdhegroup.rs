// Generated macro for FfdheGroup (struct)
macro_rules! Depcrate_msgs_ffdhe_groupsFfdheGroup {
() => {
// Module: crate::msgs::ffdhe_groups
// Provides: {"FfdheGroup"}
// Dependencies: {}
# [doc = " Parameters of an FFDHE group, with Big-endian byte order"] # [expect (clippy :: exhaustive_structs)] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub struct FfdheGroup < 'a > { pub p : & 'a [u8] , pub g : & 'a [u8] , }
};
}
