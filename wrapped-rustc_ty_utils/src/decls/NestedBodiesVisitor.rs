macro_rules! NestedBodiesVisitor {
    () => {
        struct NestedBodiesVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , root_def_id : DefId , nested_bodies : Vec < LocalDefId > , }
    };
}

NestedBodiesVisitor!()