// Generated macro for other_14 (other)
macro_rules! Depcrate_access_controlother_14 {
() => {
// Module: crate::access_control
// Provides: {"other_14"}
// Dependencies: {}
extern "C" { pub fn SecAccessControlGetTypeID () -> CFTypeID ; pub fn SecAccessControlCreateWithFlags (allocator : CFAllocatorRef , protection : CFTypeRef , flags : CFOptionFlags , error : * mut CFErrorRef ,) -> SecAccessControlRef ; }
};
}
