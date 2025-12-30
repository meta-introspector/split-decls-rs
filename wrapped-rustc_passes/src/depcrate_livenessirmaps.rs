// Generated macro for IrMaps (struct)
macro_rules! Depcrate_livenessIrMaps {
() => {
// Module: crate::liveness
// Provides: {"IrMaps"}
// Dependencies: {}
struct IrMaps < 'tcx > { tcx : TyCtxt < 'tcx > , live_node_map : HirIdMap < LiveNode > , variable_map : HirIdMap < Variable > , capture_info_map : HirIdMap < Rc < Vec < CaptureInfo > > > , var_kinds : IndexVec < Variable , VarKind > , lnks : IndexVec < LiveNode , LiveNodeKind > , }
};
}
