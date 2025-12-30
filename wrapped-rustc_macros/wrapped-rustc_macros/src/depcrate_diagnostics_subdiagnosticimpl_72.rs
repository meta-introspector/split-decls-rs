// Generated macro for impl_72 (impl)
macro_rules! Depcrate_diagnostics_subdiagnosticimpl_72 {
() => {
// Module: crate::diagnostics::subdiagnostic
// Provides: {"impl_72"}
// Dependencies: {}
impl < 'a > FromIterator < & 'a SubdiagnosticKind > for KindsStatistics { fn from_iter < T : IntoIterator < Item = & 'a SubdiagnosticKind > > (kinds : T) -> Self { let mut ret = Self { has_multipart_suggestion : false , all_multipart_suggestions : true , has_normal_suggestion : false , all_applicabilities_static : true , } ; for kind in kinds { if let SubdiagnosticKind :: MultipartSuggestion { applicability : None , .. } | SubdiagnosticKind :: Suggestion { applicability : None , .. } = kind { ret . all_applicabilities_static = false ; } if let SubdiagnosticKind :: MultipartSuggestion { .. } = kind { ret . has_multipart_suggestion = true ; } else { ret . all_multipart_suggestions = false ; } if let SubdiagnosticKind :: Suggestion { .. } = kind { ret . has_normal_suggestion = true ; } } ret } }
};
}
