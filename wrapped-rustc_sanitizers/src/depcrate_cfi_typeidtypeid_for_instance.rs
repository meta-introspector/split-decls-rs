// Generated macro for typeid_for_instance (function)
macro_rules! Depcrate_cfi_typeidtypeid_for_instance {
() => {
// Module: crate::cfi::typeid
// Provides: {"typeid_for_instance"}
// Dependencies: {}
# [doc = " Returns a type metadata identifier for the specified Instance."] pub fn typeid_for_instance < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , options : TypeIdOptions ,) -> String { itanium_cxx_abi :: typeid_for_instance (tcx , instance , options) }
};
}
