// Generated macro for separated_by_commas (function)
macro_rules! Depcrate_corsseparated_by_commas {
() => {
// Module: crate::cors
// Provides: {"separated_by_commas"}
// Dependencies: {}
fn separated_by_commas < I > (mut iter : I) -> Option < HeaderValue > where I : Iterator < Item = HeaderValue > , { match iter . next () { Some (fst) => { let mut result = BytesMut :: from (fst . as_bytes ()) ; for val in iter { result . reserve (val . len () + 1) ; result . put_u8 (b',') ; result . extend_from_slice (val . as_bytes ()) ; } Some (HeaderValue :: from_maybe_shared (result . freeze ()) . unwrap ()) } None => None , } }
};
}
