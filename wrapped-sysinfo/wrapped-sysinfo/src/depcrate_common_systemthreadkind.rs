// Generated macro for ThreadKind (enum)
macro_rules! Depcrate_common_systemThreadKind {
() => {
// Module: crate::common::system
// Provides: {"ThreadKind"}
// Dependencies: {}
# [doc = " Enum describing the different kind of threads."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub enum ThreadKind { # [doc = " Kernel thread."] Kernel , # [doc = " User thread."] Userland , }
};
}
