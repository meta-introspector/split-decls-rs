// Generated macro for classify_arg (function)
macro_rules! Depcrate_callconv_mips64classify_arg {
() => {
// Module: crate::callconv::mips64
// Provides: {"classify_arg"}
// Dependencies: {}
fn classify_arg < 'a , Ty , C > (cx : & C , arg : & mut ArgAbi < 'a , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout , { if ! arg . layout . is_aggregate () { extend_integer_width_mips (arg , 64) ; return ; } let dl = cx . data_layout () ; let size = arg . layout . size ; let mut prefix = [None ; 8] ; let mut prefix_index = 0 ; match arg . layout . fields { FieldsShape :: Primitive => unreachable ! () , FieldsShape :: Array { .. } => { arg . make_indirect () ; return ; } FieldsShape :: Union (_) => { } FieldsShape :: Arbitrary { .. } => { let mut last_offset = Size :: ZERO ; for i in 0 .. arg . layout . fields . count () { let field = arg . layout . field (cx , i) ; let offset = arg . layout . fields . offset (i) ; if let BackendRepr :: Scalar (scalar) = field . backend_repr { if scalar . primitive () == Primitive :: Float (Float :: F64) { if offset . is_aligned (dl . f64_align . abi) { assert ! (last_offset . is_aligned (dl . f64_align . abi)) ; for _ in 0 .. ((offset - last_offset) . bits () / 64) . min ((prefix . len () - prefix_index) as u64) { prefix [prefix_index] = Some (Reg :: i64 ()) ; prefix_index += 1 ; } if prefix_index == prefix . len () { break ; } prefix [prefix_index] = Some (Reg :: f64 ()) ; prefix_index += 1 ; last_offset = offset + Reg :: f64 () . size ; } } } } } } ; let rest_size = size - Size :: from_bytes (8) * prefix_index as u64 ; arg . cast_to (CastTarget :: prefixed (prefix , Uniform :: new (Reg :: i64 () , rest_size))) ; }
};
}
