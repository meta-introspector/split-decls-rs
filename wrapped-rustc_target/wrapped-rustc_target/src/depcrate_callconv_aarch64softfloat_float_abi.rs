// Generated macro for softfloat_float_abi (function)
macro_rules! Depcrate_callconv_aarch64softfloat_float_abi {
() => {
// Module: crate::callconv::aarch64
// Provides: {"softfloat_float_abi"}
// Dependencies: {}
fn softfloat_float_abi < Ty > (target : & Target , arg : & mut ArgAbi < '_ , Ty >) { if target . abi != "softfloat" { return ; } if let BackendRepr :: Scalar (s) = arg . layout . backend_repr && let Primitive :: Float (f) = s . primitive () { arg . cast_to (Reg { kind : RegKind :: Integer , size : f . size () }) ; } else if let BackendRepr :: ScalarPair (s1 , s2) = arg . layout . backend_repr && (matches ! (s1 . primitive () , Primitive :: Float (_)) || matches ! (s2 . primitive () , Primitive :: Float (_))) { if arg . layout . size . bits () <= target . pointer_width . into () { arg . cast_to (Reg { kind : RegKind :: Integer , size : arg . layout . size }) ; } else { arg . make_indirect () ; } } }
};
}
