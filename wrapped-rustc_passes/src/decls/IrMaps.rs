macro_rules! deps {
    () => {
        VarKind!();
        LiveNodeKind!();
        CaptureInfo!();
    };
}

macro_rules! IrMaps {
    () => {
        deps!();
        struct IrMaps < 'tcx > { tcx : TyCtxt < 'tcx > , live_node_map : HirIdMap < LiveNode > , variable_map : HirIdMap < Variable > , capture_info_map : HirIdMap < Rc < Vec < CaptureInfo > > > , var_kinds : IndexVec < Variable , VarKind > , lnks : IndexVec < LiveNode , LiveNodeKind > , }
    };
}

IrMaps!()