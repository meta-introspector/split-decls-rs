use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Type privacy visitor, checks types for privacy and reports violations.
///
/// Both explicitly written types and inferred types of expressions and patterns are checked.
/// Checks are performed on "semantic" types regardless of names and their hygiene.
struct TypePrivacyVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    module_def_id: LocalModDefId,
    maybe_typeck_results: Option<&'tcx ty::TypeckResults<'tcx>>,
    span: Span,
}
