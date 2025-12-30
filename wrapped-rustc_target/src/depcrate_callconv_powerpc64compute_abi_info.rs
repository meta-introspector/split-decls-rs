// Generated macro for compute_abi_info (function)
macro_rules! Depcrate_callconv_powerpc64compute_abi_info {
() => {
// Module: crate::callconv::powerpc64
// Provides: {"compute_abi_info"}
// Dependencies: {}
pub (crate) fn compute_abi_info < 'a , Ty , C > (cx : & C , fn_abi : & mut FnAbi < 'a , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout + HasTargetSpec , { let abi = if cx . target_spec () . env == "musl" || cx . target_spec () . os == "freebsd" { ELFv2 } else if cx . target_spec () . os == "aix" { AIX } else { match cx . data_layout () . endian { Endian :: Big => ELFv1 , Endian :: Little => ELFv2 , } } ; classify (cx , & mut fn_abi . ret , abi , true) ; for arg in fn_abi . args . iter_mut () { classify (cx , arg , abi , false) ; } }
};
}
