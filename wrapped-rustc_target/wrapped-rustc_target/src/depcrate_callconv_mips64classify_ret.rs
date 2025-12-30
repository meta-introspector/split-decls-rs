// Generated macro for classify_ret (function)
macro_rules! Depcrate_callconv_mips64classify_ret {
() => {
// Module: crate::callconv::mips64
// Provides: {"classify_ret"}
// Dependencies: {}
fn classify_ret < 'a , Ty , C > (cx : & C , ret : & mut ArgAbi < 'a , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout , { if ! ret . layout . is_aggregate () { extend_integer_width_mips (ret , 64) ; return ; } let size = ret . layout . size ; let bits = size . bits () ; if bits <= 128 { if let FieldsShape :: Arbitrary { .. } = ret . layout . fields { if ret . layout . fields . count () == 1 { if let Some (reg) = float_reg (cx , ret , 0) { ret . cast_to (reg) ; return ; } } else if ret . layout . fields . count () == 2 && let Some (reg0) = float_reg (cx , ret , 0) && let Some (reg1) = float_reg (cx , ret , 1) { ret . cast_to (CastTarget :: pair (reg0 , reg1)) ; return ; } } ret . cast_to (Uniform :: new (Reg :: i64 () , size)) ; } else { ret . make_indirect () ; } }
};
}
