// Generated macro for classify_arg (function)
macro_rules! Depcrate_callconv_amdgpuclassify_arg {
() => {
// Module: crate::callconv::amdgpu
// Provides: {"classify_arg"}
// Dependencies: {}
fn classify_arg < 'a , Ty , C > (_cx : & C , arg : & mut ArgAbi < 'a , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout , { arg . extend_integer_width_to (32) ; }
};
}
