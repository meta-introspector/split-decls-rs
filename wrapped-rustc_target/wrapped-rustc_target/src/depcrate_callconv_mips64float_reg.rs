// Generated macro for float_reg (function)
macro_rules! Depcrate_callconv_mips64float_reg {
() => {
// Module: crate::callconv::mips64
// Provides: {"float_reg"}
// Dependencies: {}
fn float_reg < 'a , Ty , C > (cx : & C , ret : & ArgAbi < 'a , Ty > , i : usize) -> Option < Reg > where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout , { match ret . layout . field (cx , i) . backend_repr { BackendRepr :: Scalar (scalar) => match scalar . primitive () { Primitive :: Float (Float :: F32) => Some (Reg :: f32 ()) , Primitive :: Float (Float :: F64) => Some (Reg :: f64 ()) , _ => None , } , _ => None , } }
};
}
