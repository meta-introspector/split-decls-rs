// Generated macro for DictKey (enum)
macro_rules! Depcrate_cfi_typeid_itanium_cxx_abi_encodeDictKey {
() => {
// Module: crate::cfi::typeid::itanium_cxx_abi::encode
// Provides: {"DictKey"}
// Dependencies: {}
# [doc = " Substitution dictionary key."] # [derive (Eq , Hash , PartialEq)] pub (crate) enum DictKey < 'tcx > { Ty (Ty < 'tcx > , TyQ) , Region (Region < 'tcx >) , Const (Const < 'tcx >) , Predicate (ExistentialPredicate < 'tcx >) , }
};
}
