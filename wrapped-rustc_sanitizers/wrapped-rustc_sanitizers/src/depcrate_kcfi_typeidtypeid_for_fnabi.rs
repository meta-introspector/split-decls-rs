// Generated macro for typeid_for_fnabi (function)
macro_rules! Depcrate_kcfi_typeidtypeid_for_fnabi {
() => {
// Module: crate::kcfi::typeid
// Provides: {"typeid_for_fnabi"}
// Dependencies: {}
# [doc = " Returns a KCFI type metadata identifier for the specified FnAbi."] pub fn typeid_for_fnabi < 'tcx > (tcx : TyCtxt < 'tcx > , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > > , options : TypeIdOptions ,) -> u32 { let mut hash : XxHash64 = Default :: default () ; hash . write (itanium_cxx_abi :: typeid_for_fnabi (tcx , fn_abi , options) . as_bytes ()) ; hash . finish () as u32 }
};
}
