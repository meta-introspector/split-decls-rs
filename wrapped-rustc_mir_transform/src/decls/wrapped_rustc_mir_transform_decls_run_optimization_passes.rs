use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn run_optimization_passes<'tcx>(tcx: TyCtxt<'tcx>, body: &mut Body<'tcx>) {
    fn o1<T>(x: T) -> WithMinOptLevel<T> {
        WithMinOptLevel(1, x)
    }
    let def_id = body.source.def_id();
    let optimizations = if tcx.def_kind(def_id).has_codegen_attrs()
        && tcx.codegen_fn_attrs(def_id).optimize.do_not_optimize()
    {
        pm::Optimizations::Suppressed
    } else {
        pm::Optimizations::Allowed
    };
    pm::run_passes(
        tcx,
        body,
        &[
            &check_alignment::CheckAlignment,
            &check_null::CheckNull,
            &check_enums::CheckEnums,
            &lower_slice_len::LowerSliceLenCalls,
            &instsimplify::InstSimplify::BeforeInline,
            &inline::ForceInline,
            &inline::Inline,
            &remove_storage_markers::RemoveStorageMarkers,
            &remove_zsts::RemoveZsts,
            &remove_unneeded_drops::RemoveUnneededDrops,
            &unreachable_enum_branching::UnreachableEnumBranching,
            &unreachable_prop::UnreachablePropagation,
            &o1(simplify::SimplifyCfg::AfterUnreachableEnumBranching),
            &multiple_return_terminators::MultipleReturnTerminators,
            &instsimplify::InstSimplify::AfterSimplifyCfg,
            &o1(simplify_branches::SimplifyConstCondition::AfterInstSimplify),
            &ref_prop::ReferencePropagation,
            &sroa::ScalarReplacementOfAggregates,
            &simplify::SimplifyLocals::BeforeConstProp,
            &dead_store_elimination::DeadStoreElimination::Initial,
            &gvn::GVN,
            &simplify::SimplifyLocals::AfterGVN,
            &match_branches::MatchBranchSimplification,
            &dataflow_const_prop::DataflowConstProp,
            &single_use_consts::SingleUseConsts,
            &o1(simplify_branches::SimplifyConstCondition::AfterConstProp),
            &jump_threading::JumpThreading,
            &early_otherwise_branch::EarlyOtherwiseBranch,
            &simplify_comparison_integral::SimplifyComparisonIntegral,
            &o1(simplify_branches::SimplifyConstCondition::Final),
            &o1(remove_noop_landing_pads::RemoveNoopLandingPads),
            &o1(simplify::SimplifyCfg::Final),
            &strip_debuginfo::StripDebugInfo,
            &copy_prop::CopyProp,
            &dead_store_elimination::DeadStoreElimination::Final,
            &dest_prop::DestinationPropagation,
            &simplify::SimplifyLocals::Final,
            &multiple_return_terminators::MultipleReturnTerminators,
            &large_enums::EnumSizeOpt { discrepancy: 128 },
            &add_call_guards::CriticalCallEdges,
            &prettify::ReorderBasicBlocks,
            &prettify::ReorderLocals,
            &dump_mir::Marker("PreCodegen"),
        ],
        Some(MirPhase::Runtime(RuntimePhase::Optimized)),
        optimizations,
    );
}
