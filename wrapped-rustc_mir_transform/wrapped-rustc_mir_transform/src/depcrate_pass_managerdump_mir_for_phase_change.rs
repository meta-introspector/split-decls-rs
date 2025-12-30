// Generated macro for dump_mir_for_phase_change (function)
macro_rules! Depcrate_pass_managerdump_mir_for_phase_change {
() => {
// Module: crate::pass_manager
// Provides: {"dump_mir_for_phase_change"}
// Dependencies: {}
pub (super) fn dump_mir_for_phase_change < 'tcx > (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx >) { assert_eq ! (body . pass_count , 0) ; if let Some (dumper) = MirDumper :: new (tcx , body . phase . name () , body) { dumper . set_show_pass_num () . set_disambiguator (& "after") . dump_mir (body) } }
};
}
