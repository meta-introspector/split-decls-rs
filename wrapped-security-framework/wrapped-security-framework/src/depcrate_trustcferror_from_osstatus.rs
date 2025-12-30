// Generated macro for cferror_from_osstatus (function)
macro_rules! Depcrate_trustcferror_from_osstatus {
() => {
// Module: crate::trust
// Provides: {"cferror_from_osstatus"}
// Dependencies: {}
fn cferror_from_osstatus (code : core_foundation_sys :: base :: OSStatus) -> CFError { unsafe { let error = CFErrorCreate (ptr :: null_mut () , core_foundation_sys :: error :: kCFErrorDomainOSStatus , code as _ , ptr :: null_mut ()) ; assert ! (! error . is_null ()) ; CFError :: wrap_under_create_rule (error) } }
};
}
