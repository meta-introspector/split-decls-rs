use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Obtain just the main MIR (no promoteds) and run some cleanups on it. This also runs
/// mir borrowck *before* doing so in order to ensure that borrowck can be run and doesn't
/// end up missing the source MIR due to stealing happening.
fn mir_drops_elaborated_and_const_checked(tcx: TyCtxt<'_>, def: LocalDefId) -> &Steal<Body<'_>> {
    if tcx.is_coroutine(def.to_def_id()) {
        tcx.ensure_done().mir_coroutine_witnesses(def);
    }
    let tainted_by_errors = if !tcx.is_synthetic_mir(def) {
        tcx.mir_borrowck(tcx.typeck_root_def_id(def.to_def_id()).expect_local())
            .err()
    } else {
        None
    };
    let is_fn_like = tcx.def_kind(def).is_fn_like();
    if is_fn_like {
        if pm::should_run_pass(tcx, &inline::Inline, pm::Optimizations::Allowed)
            || inline::ForceInline::should_run_pass_for_callee(tcx, def.to_def_id())
        {
            tcx.ensure_done()
                .mir_inliner_callees(ty::InstanceKind::Item(def.to_def_id()));
        }
    }
    tcx.ensure_done().check_liveness(def);
    let (body, _) = tcx.mir_promoted(def);
    let mut body = body.steal();
    if let Some(error_reported) = tainted_by_errors {
        body.tainted_by_errors = Some(error_reported);
    }
    let root = tcx.typeck_root_def_id(def.to_def_id());
    match tcx.def_kind(root) {
        DefKind::Fn
        | DefKind::AssocFn
        | DefKind::Static { .. }
        | DefKind::Const
        | DefKind::AssocConst => {
            if let Err(guar) = tcx.ensure_ok().check_well_formed(root.expect_local()) {
                body.tainted_by_errors = Some(guar);
            }
        }
        _ => {}
    }
    run_analysis_to_runtime_passes(tcx, &mut body);
    tcx.alloc_steal_mir(body)
}
