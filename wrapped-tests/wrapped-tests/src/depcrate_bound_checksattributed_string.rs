// Generated macro for attributed_string (function)
macro_rules! Depcrate_bound_checksattributed_string {
() => {
// Module: crate::bound_checks
// Provides: {"attributed_string"}
// Dependencies: {}
# [test] fn attributed_string () { let arr = unsafe { NSAttributedString :: initWithString_attributes (NSAttributedString :: alloc () , & NSString :: from_str ("foo") , Some (& NSDictionary :: new ()) ,) } ; let mut range = NSRange :: new (42 , 42) ; assert_throws ("Out of bounds" , | | unsafe { arr . attributesAtIndex_effectiveRange (100 , & mut range) }) ; assert_eq ! (range , NSRange :: new (42 , 42)) ; }
};
}
