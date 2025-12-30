// Generated macro for FIX_LINKING (static)
macro_rules! Depcrate_test_objectFIX_LINKING {
() => {
// Module: crate::test_object
// Provides: {"FIX_LINKING"}
// Dependencies: {}
# [cfg (all (target_vendor = "apple" , target_arch = "aarch64"))] # [used] static FIX_LINKING : & AnyClass = { extern "C" { # [link_name = "OBJC_CLASS_$_MyTestObject"] static CLASS : AnyClass ; } unsafe { & CLASS } } ;
};
}
