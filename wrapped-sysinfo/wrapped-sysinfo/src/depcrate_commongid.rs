// Generated macro for gid (macro)
macro_rules! Depcrate_commongid {
() => {
// Module: crate::common
// Provides: {"gid"}
// Dependencies: {}
macro_rules ! gid { ($ type : ty) => { xid ! (# [doc = " A group id wrapping a platform specific type."] # [derive (Copy)] Gid , $ type , std :: str :: FromStr) ; } ; }
};
}
