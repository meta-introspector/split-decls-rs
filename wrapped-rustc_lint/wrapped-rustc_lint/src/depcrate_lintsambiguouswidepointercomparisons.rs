// Generated macro for AmbiguousWidePointerComparisons (enum)
macro_rules! Depcrate_lintsAmbiguousWidePointerComparisons {
() => {
// Module: crate::lints
// Provides: {"AmbiguousWidePointerComparisons"}
// Dependencies: {}
# [derive (LintDiagnostic)] pub (crate) enum AmbiguousWidePointerComparisons < 'a > { # [diag (lint_ambiguous_wide_pointer_comparisons)] SpanfulEq { # [subdiagnostic] addr_suggestion : AmbiguousWidePointerComparisonsAddrSuggestion < 'a > , # [subdiagnostic] addr_metadata_suggestion : Option < AmbiguousWidePointerComparisonsAddrMetadataSuggestion < 'a > > , } , # [diag (lint_ambiguous_wide_pointer_comparisons)] SpanfulCmp { # [subdiagnostic] cast_suggestion : AmbiguousWidePointerComparisonsCastSuggestion < 'a > , # [subdiagnostic] expect_suggestion : AmbiguousWidePointerComparisonsExpectSuggestion < 'a > , } , # [diag (lint_ambiguous_wide_pointer_comparisons)] # [help (lint_addr_metadata_suggestion)] # [help (lint_addr_suggestion)] Spanless , }
};
}
