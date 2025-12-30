// Generated macro for typeid_for_instance (function)
macro_rules! Depcrate_cfi_typeid_itanium_cxx_abitypeid_for_instance {
() => {
// Module: crate::cfi::typeid::itanium_cxx_abi
// Provides: {"typeid_for_instance"}
// Dependencies: {}
# [doc = " Returns a type metadata identifier for the specified Instance using the Itanium C++ ABI with"] # [doc = " vendor extended type qualifiers and types for Rust types that are not used at the FFI boundary."] # [instrument (level = "trace" , skip (tcx))] pub fn typeid_for_instance < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , options : TypeIdOptions ,) -> String { assert ! (! instance . has_non_region_param () , "{instance:#?} must be fully monomorphic") ; let transform_ty_options = TransformTyOptions :: from_bits (options . bits ()) . unwrap_or_else (| | bug ! ("typeid_for_instance: invalid option(s) `{:?}`" , options . bits ())) ; let instance = transform_instance (tcx , instance , transform_ty_options) ; let fn_abi = tcx . fn_abi_of_instance (ty :: TypingEnv :: fully_monomorphized () . as_query_input ((instance , ty :: List :: empty ())) ,) . unwrap_or_else (| error | { bug ! ("typeid_for_instance: couldn't get fn_abi of instance {instance:?}: {error:?}") }) ; typeid_for_fnabi (tcx , fn_abi , options) }
};
}
