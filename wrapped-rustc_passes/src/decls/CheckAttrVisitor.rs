macro_rules! CheckAttrVisitor {
    () => {
        struct CheckAttrVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , abort : Cell < bool > , }
    };
}

CheckAttrVisitor!();