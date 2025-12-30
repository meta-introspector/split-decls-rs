// Generated macro for method_signature (function)
macro_rules! Depcrate_bound_checksmethod_signature {
() => {
// Module: crate::bound_checks
// Provides: {"method_signature"}
// Dependencies: {}
# [test] fn method_signature () { let sig = c"c@:@" ; let sig = NonNull :: new (sig . as_ptr () . cast_mut ()) . unwrap () ; let sig = unsafe { NSMethodSignature :: signatureWithObjCTypes (sig) } . unwrap () ; assert_throws ("index (100) out of bounds [0, 2]" , | | { sig . getArgumentTypeAtIndex (100) }) ; }
};
}
