macro_rules! deps {
    () => {
        TypePrivacyVisitor!();
        DefIdVisitor!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < 'tcx > DefIdVisitor < 'tcx > for TypePrivacyVisitor < 'tcx > { type Result = ControlFlow < () > ; fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn visit_def_id (& mut self , def_id : DefId , kind : & str , descr : & dyn fmt :: Display ,) -> Self :: Result { if self . check_def_id (def_id , kind , descr) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } } }
    };
}

impl_39!()