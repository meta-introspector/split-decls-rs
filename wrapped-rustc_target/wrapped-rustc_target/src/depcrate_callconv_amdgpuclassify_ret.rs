// Generated macro for classify_ret (function)
macro_rules! Depcrate_callconv_amdgpuclassify_ret {
() => {
// Module: crate::callconv::amdgpu
// Provides: {"classify_ret"}
// Dependencies: {}
fn classify_ret < 'a , Ty , C > (_cx : & C , ret : & mut ArgAbi < 'a , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout , { ret . extend_integer_width_to (32) ; }
};
}
