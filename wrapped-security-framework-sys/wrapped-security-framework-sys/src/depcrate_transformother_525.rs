// Generated macro for other_525 (other)
macro_rules! Depcrate_transformother_525 {
() => {
// Module: crate::transform
// Provides: {"other_525"}
// Dependencies: {}
extern "C" { pub static kSecTransformInputAttributeName : CFStringRef ; pub fn SecTransformGetTypeID () -> CFTypeID ; pub fn SecTransformSetAttribute (transformRef : SecTransformRef , key : CFStringRef , value : CFTypeRef , error : * mut CFErrorRef ,) -> Boolean ; pub fn SecTransformExecute (transformRef : SecTransformRef , errorRef : * mut CFErrorRef ,) -> CFTypeRef ; }
};
}
