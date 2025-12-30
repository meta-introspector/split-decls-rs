// Generated macro for is_homogeneous_aggregate (function)
macro_rules! Depcrate_callconv_aarch64is_homogeneous_aggregate {
() => {
// Module: crate::callconv::aarch64
// Provides: {"is_homogeneous_aggregate"}
// Dependencies: {}
fn is_homogeneous_aggregate < 'a , Ty , C > (cx : & C , arg : & mut ArgAbi < 'a , Ty >) -> Option < Uniform > where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout + HasTargetSpec , { arg . layout . homogeneous_aggregate (cx) . ok () . and_then (| ha | ha . unit ()) . and_then (| unit | { let size = arg . layout . size ; if size > unit . size . checked_mul (4 , cx) . unwrap () { return None ; } let valid_unit = match unit . kind { RegKind :: Integer => false , RegKind :: Float => cx . target_spec () . abi != "softfloat" , RegKind :: Vector => size . bits () == 64 || size . bits () == 128 , } ; valid_unit . then_some (Uniform :: consecutive (unit , size)) }) }
};
}
