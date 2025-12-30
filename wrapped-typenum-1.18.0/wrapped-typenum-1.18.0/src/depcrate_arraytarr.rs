// Generated macro for tarr (macro)
macro_rules! Depcrate_arraytarr {
() => {
// Module: crate::array
// Provides: {"tarr"}
// Dependencies: {}
# [doc = " Create a new type-level array. Only usable on Rust 1.13.0 or newer."] # [doc = ""] # [doc = " There's not a whole lot you can do with it right now."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " #[macro_use]"] # [doc = " extern crate typenum;"] # [doc = " use typenum::consts::*;"] # [doc = ""] # [doc = " type Array = tarr![P3, N4, Z0, P38];"] # [doc = " # fn main() { let _: Array; }"] # [macro_export] macro_rules ! tarr { () => ($ crate :: ATerm) ; ($ n : ty) => ($ crate :: TArr <$ n , $ crate :: ATerm >) ; ($ n : ty ,) => ($ crate :: TArr <$ n , $ crate :: ATerm >) ; ($ n : ty , $ ($ tail : ty) ,+) => ($ crate :: TArr <$ n , tarr ! [$ ($ tail) ,+] >) ; ($ n : ty , $ ($ tail : ty) ,+,) => ($ crate :: TArr <$ n , tarr ! [$ ($ tail) ,+] >) ; ($ n : ty | $ rest : ty) => ($ crate :: TArr <$ n , $ rest >) ; ($ n : ty , $ ($ tail : ty) ,+ | $ rest : ty) => ($ crate :: TArr <$ n , ta ! [$ ($ tail) ,+ | $ rest] >) ; }
};
}
