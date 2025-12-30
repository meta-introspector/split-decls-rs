// Generated macro for impl_1613 (impl)
macro_rules! Depcrate_errorimpl_1613 {
() => {
// Module: crate::error
// Provides: {"impl_1613"}
// Dependencies: {}
impl fmt :: Display for CertificateError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { # [cfg (feature = "std")] Self :: NotValidForNameContext { expected , presented , } => { write ! (f , "certificate not valid for name {:?}; certificate " , expected . to_str ()) ? ; match presented . as_slice () { & [] => write ! (f , "is not valid for any names (according to its subjectAltName extension)") , [one] => write ! (f , "is only valid for {one}") , many => { write ! (f , "is only valid for ") ? ; let n = many . len () ; let all_but_last = & many [.. n - 1] ; let last = & many [n - 1] ; for (i , name) in all_but_last . iter () . enumerate () { write ! (f , "{name}") ? ; if i < n - 2 { write ! (f , ", ") ? ; } } write ! (f , " or {last}") } } } Self :: ExpiredContext { time , not_after } => write ! (f , "certificate expired: verification time {} (UNIX), \
                 but certificate is not valid after {} \
                 ({} seconds ago)" , time . as_secs () , not_after . as_secs () , time . as_secs () . saturating_sub (not_after . as_secs ())) , Self :: NotValidYetContext { time , not_before } => write ! (f , "certificate not valid yet: verification time {} (UNIX), \
                 but certificate is not valid before {} \
                 ({} seconds in future)" , time . as_secs () , not_before . as_secs () , not_before . as_secs () . saturating_sub (time . as_secs ())) , Self :: ExpiredRevocationListContext { time , next_update } => write ! (f , "certificate revocation list expired: \
                 verification time {} (UNIX), \
                 but CRL is not valid after {} \
                 ({} seconds ago)" , time . as_secs () , next_update . as_secs () , time . as_secs () . saturating_sub (next_update . as_secs ())) , Self :: InvalidPurposeContext { required , presented , } => { write ! (f , "certificate does not allow extended key usage for {required}, allows ") ? ; for (i , eku) in presented . iter () . enumerate () { if i > 0 { write ! (f , ", ") ? ; } write ! (f , "{eku}") ? ; } Ok (()) } Self :: Other (other) => write ! (f , "{other}") , other => write ! (f , "{other:?}") , } } }
};
}
