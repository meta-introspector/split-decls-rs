// Generated macro for impl_353 (impl)
macro_rules! Depcrate_verify_certimpl_353 {
() => {
// Module: crate::verify_cert
// Provides: {"impl_353"}
// Dependencies: {}
impl Iterator for OidDecoder < '_ > { type Item = usize ; fn next (& mut self) -> Option < Self :: Item > { if let Some (next) = self . left . take () { return Some (next) ; } let mut cur = 0 ; for (i , & byte) in self . encoded . iter () . enumerate () { cur = (cur << 7) + usize :: from (byte & 0x7f) ; if byte & 0x80 > 0 { continue ; } if ! self . first { self . encoded = & self . encoded [i + 1 ..] ; return Some (cur) ; } let (cur , next) = match cur { ..= 39 => (0 , cur) , 40 ..= 79 => (1 , cur - 40) , _ => (2 , cur - 80) , } ; self . encoded = & self . encoded [i + 1 ..] ; self . first = false ; self . left = Some (next) ; return Some (cur) ; } None } }
};
}
