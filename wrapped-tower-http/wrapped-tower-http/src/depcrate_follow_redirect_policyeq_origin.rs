// Generated macro for eq_origin (function)
macro_rules! Depcrate_follow_redirect_policyeq_origin {
() => {
// Module: crate::follow_redirect::policy
// Provides: {"eq_origin"}
// Dependencies: {}
# [doc = " Compares the origins of two URIs as per RFC 6454 sections 4. through 5."] fn eq_origin (lhs : & Uri , rhs : & Uri) -> bool { let default_port = match (lhs . scheme () , rhs . scheme ()) { (Some (l) , Some (r)) if l == r => { if l == & Scheme :: HTTP { 80 } else if l == & Scheme :: HTTPS { 443 } else { return false ; } } _ => return false , } ; match (lhs . host () , rhs . host ()) { (Some (l) , Some (r)) if l == r => { } _ => return false , } lhs . port_u16 () . unwrap_or (default_port) == rhs . port_u16 () . unwrap_or (default_port) }
};
}
