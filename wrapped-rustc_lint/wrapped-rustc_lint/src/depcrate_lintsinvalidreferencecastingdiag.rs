// Generated macro for InvalidReferenceCastingDiag (enum)
macro_rules! Depcrate_lintsInvalidReferenceCastingDiag {
() => {
// Module: crate::lints
// Provides: {"InvalidReferenceCastingDiag"}
// Dependencies: {}
# [derive (LintDiagnostic)] pub (crate) enum InvalidReferenceCastingDiag < 'tcx > { # [diag (lint_invalid_reference_casting_borrow_as_mut)] # [note (lint_invalid_reference_casting_note_book)] BorrowAsMut { # [label] orig_cast : Option < Span > , # [note (lint_invalid_reference_casting_note_ty_has_interior_mutability)] ty_has_interior_mutability : bool , } , # [diag (lint_invalid_reference_casting_assign_to_ref)] # [note (lint_invalid_reference_casting_note_book)] AssignToRef { # [label] orig_cast : Option < Span > , # [note (lint_invalid_reference_casting_note_ty_has_interior_mutability)] ty_has_interior_mutability : bool , } , # [diag (lint_invalid_reference_casting_bigger_layout)] # [note (lint_layout)] BiggerLayout { # [label] orig_cast : Option < Span > , # [label (lint_alloc)] alloc : Span , from_ty : Ty < 'tcx > , from_size : u64 , to_ty : Ty < 'tcx > , to_size : u64 , } , }
};
}
