use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub struct MockTraitFixer<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub config: trait_fixer_rules::Config,
    pub fixes: Vec<Fix<Span, DefId, DefId>>,
}
