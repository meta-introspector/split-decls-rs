use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The embargo visitor, used to determine the exports of the AST.
struct EmbargoVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    /// Effective visibilities for reachable nodes.
    effective_visibilities: EffectiveVisibilities,
    /// A set of pairs corresponding to modules, where the first module is
    /// reachable via a macro that's defined in the second module. This cannot
    /// be represented as reachable because it can't handle the following case:
    ///
    /// pub mod n {                         // Should be `Public`
    ///     pub mod p {              // Should *not* be accessible
    ///         pub fn f() -> i32 { 12 }    // Must be `Reachable`
    ///     }
    /// }
    /// pub macro m() {
    ///     n::p::f()
    /// }
    macro_reachable: FxHashSet<(LocalModDefId, LocalModDefId)>,
    /// Has something changed in the level map?
    changed: bool,
}
