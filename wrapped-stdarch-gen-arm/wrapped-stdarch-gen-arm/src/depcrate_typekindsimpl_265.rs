// Generated macro for impl_265 (impl)
macro_rules! Depcrate_typekindsimpl_265 {
() => {
// Module: crate::typekinds
// Provides: {"impl_265"}
// Dependencies: {}
impl FromStr for TypeKindOptions { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let mut result = Self :: default () ; for kind in s . bytes () { match kind { b'f' => result . f = true , b's' => result . s = true , b'u' => result . u = true , b'p' => result . p = true , _ => { return Err (format ! ("unknown type kind: {}" , char :: from (kind))) ; } } } Ok (result) } }
};
}
