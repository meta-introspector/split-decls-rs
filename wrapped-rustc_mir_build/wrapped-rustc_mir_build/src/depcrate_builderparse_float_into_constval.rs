// Generated macro for parse_float_into_constval (function)
macro_rules! Depcrate_builderparse_float_into_constval {
() => {
// Module: crate::builder
// Provides: {"parse_float_into_constval"}
// Dependencies: {}
fn parse_float_into_constval (num : Symbol , float_ty : ty :: FloatTy , neg : bool) -> Option < ConstValue > { parse_float_into_scalar (num , float_ty , neg) . map (| s | ConstValue :: Scalar (s . into ())) }
};
}
