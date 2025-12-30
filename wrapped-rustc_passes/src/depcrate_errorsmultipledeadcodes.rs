// Generated macro for MultipleDeadCodes (enum)
macro_rules! Depcrate_errorsMultipleDeadCodes {
() => {
// Module: crate::errors
// Provides: {"MultipleDeadCodes"}
// Dependencies: {}
# [derive (LintDiagnostic)] pub (crate) enum MultipleDeadCodes < 'tcx > { # [diag (passes_dead_codes)] DeadCodes { multiple : bool , num : usize , descr : & 'tcx str , participle : & 'tcx str , name_list : DiagSymbolList , # [subdiagnostic] enum_variants_with_same_name : Vec < EnumVariantSameName < 'tcx > > , # [subdiagnostic] parent_info : Option < ParentInfo < 'tcx > > , # [subdiagnostic] ignored_derived_impls : Option < IgnoredDerivedImpls > , } , # [diag (passes_dead_codes)] UnusedTupleStructFields { multiple : bool , num : usize , descr : & 'tcx str , participle : & 'tcx str , name_list : DiagSymbolList , # [subdiagnostic] change_fields_suggestion : ChangeFields , # [subdiagnostic] parent_info : Option < ParentInfo < 'tcx > > , # [subdiagnostic] ignored_derived_impls : Option < IgnoredDerivedImpls > , } , }
};
}
