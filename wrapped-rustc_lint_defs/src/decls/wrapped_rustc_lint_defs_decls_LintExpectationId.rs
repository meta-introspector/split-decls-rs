use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Each lint expectation has a `LintExpectationId` assigned by the `LintLevelsBuilder`.
/// Expected diagnostics get the lint level `Expect` which stores the `LintExpectationId`
/// to match it with the actual expectation later on.
///
/// The `LintExpectationId` has to be stable between compilations, as diagnostic
/// instances might be loaded from cache. Lint messages can be emitted during an
/// `EarlyLintPass` operating on the AST and during a `LateLintPass` traversing the
/// HIR tree. The AST doesn't have enough information to create a stable id. The
/// `LintExpectationId` will instead store the [`AttrId`] defining the expectation.
/// These `LintExpectationId` will be updated to use the stable [`HirId`] once the
/// AST has been lowered. The transformation is done by the `LintLevelsBuilder`
///
/// Each lint inside the `expect` attribute is tracked individually, the `lint_index`
/// identifies the lint inside the attribute and ensures that the IDs are unique.
///
/// The index values have a type of `u16` to reduce the size of the `LintExpectationId`.
/// It's reasonable to assume that no user will define 2^16 attributes on one node or
/// have that amount of lints listed. `u16` values should therefore suffice.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Encodable, Decodable)]
pub enum LintExpectationId {
    /// Used for lints emitted during the `EarlyLintPass`. This id is not
    /// hash stable and should not be cached.
    Unstable { attr_id: AttrId, lint_index: Option<u16> },
    /// The [`HirId`] that the lint expectation is attached to. This id is
    /// stable and can be cached. The additional index ensures that nodes with
    /// several expectations can correctly match diagnostics to the individual
    /// expectation.
    Stable { hir_id: HirId, attr_index: u16, lint_index: Option<u16> },
}
