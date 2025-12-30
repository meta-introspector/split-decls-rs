// Generated macro for parse_float_into_scalar (function)
macro_rules! Depcrate_builderparse_float_into_scalar {
() => {
// Module: crate::builder
// Provides: {"parse_float_into_scalar"}
// Dependencies: {}
pub (crate) fn parse_float_into_scalar (num : Symbol , float_ty : ty :: FloatTy , neg : bool ,) -> Option < ScalarInt > { let num = num . as_str () ; match float_ty { ty :: FloatTy :: F16 => { let mut f = num . parse :: < Half > () . ok () ? ; if neg { f = - f ; } Some (ScalarInt :: from (f)) } ty :: FloatTy :: F32 => { let Ok (rust_f) = num . parse :: < f32 > () else { return None } ; let mut f = num . parse :: < Single > () . unwrap_or_else (| e | panic ! ("apfloat::ieee::Single failed to parse `{num}`: {e:?}")) ; assert ! (u128 :: from (rust_f . to_bits ()) == f . to_bits () , "apfloat::ieee::Single gave different result for `{}`: \
                 {}({:#x}) vs Rust's {}({:#x})" , rust_f , f , f . to_bits () , Single :: from_bits (rust_f . to_bits () . into ()) , rust_f . to_bits ()) ; if neg { f = - f ; } Some (ScalarInt :: from (f)) } ty :: FloatTy :: F64 => { let Ok (rust_f) = num . parse :: < f64 > () else { return None } ; let mut f = num . parse :: < Double > () . unwrap_or_else (| e | panic ! ("apfloat::ieee::Double failed to parse `{num}`: {e:?}")) ; assert ! (u128 :: from (rust_f . to_bits ()) == f . to_bits () , "apfloat::ieee::Double gave different result for `{}`: \
                 {}({:#x}) vs Rust's {}({:#x})" , rust_f , f , f . to_bits () , Double :: from_bits (rust_f . to_bits () . into ()) , rust_f . to_bits ()) ; if neg { f = - f ; } Some (ScalarInt :: from (f)) } ty :: FloatTy :: F128 => { let mut f = num . parse :: < Quad > () . ok () ? ; if neg { f = - f ; } Some (ScalarInt :: from (f)) } } }
};
}
