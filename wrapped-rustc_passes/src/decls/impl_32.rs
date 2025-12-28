macro_rules! deps {
    () => {
        ImplsOrderVisitor!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < 'tcx > ImplsOrderVisitor < 'tcx > { fn new (tcx : TyCtxt < 'tcx >) -> ImplsOrderVisitor < 'tcx > { ImplsOrderVisitor { tcx , order : Default :: default () } } }
    };
}

impl_32!()