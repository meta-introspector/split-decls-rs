macro_rules! dump_mir_for_phase_change {
    () => {
        pub (super) fn dump_mir_for_phase_change < 'tcx > (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx >) { assert_eq ! (body . pass_count , 0) ; if let Some (dumper) = MirDumper :: new (tcx , body . phase . name () , body) { dumper . set_show_pass_num () . set_disambiguator (& "after") . dump_mir (body) } }
    };
}

dump_mir_for_phase_change!()