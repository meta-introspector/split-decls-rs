// Generated macro for typeid_for_fnabi (function)
macro_rules! Depcrate_cfi_typeidtypeid_for_fnabi {
() => {
// Module: crate::cfi::typeid
// Provides: {"typeid_for_fnabi"}
// Dependencies: {}
# [doc = " Returns a type metadata identifier for the specified FnAbi."] pub fn typeid_for_fnabi < 'tcx > (tcx : TyCtxt < 'tcx > , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > > , options : TypeIdOptions ,) -> String { itanium_cxx_abi :: typeid_for_fnabi (tcx , fn_abi , options) }
};
}
