use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Fetch all the promoteds of an item and prepare their MIR bodies to be ready for
/// constant evaluation once all generic parameters become known.
fn promoted_mir(tcx: TyCtxt<'_>, def: LocalDefId) -> &IndexVec<Promoted, Body<'_>> {
    if tcx.is_constructor(def.to_def_id()) {
        return tcx.arena.alloc(IndexVec::new());
    }
    if !tcx.is_synthetic_mir(def) {
        tcx.ensure_done()
            .mir_borrowck(tcx.typeck_root_def_id(def.to_def_id()).expect_local());
    }
    let mut promoted = tcx.mir_promoted(def).1.steal();
    for body in &mut promoted {
        run_analysis_to_runtime_passes(tcx, body);
    }
    tcx.arena.alloc(promoted)
}
