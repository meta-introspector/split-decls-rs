macro_rules! deps {
    () => {
        ReachableContext!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        impl < 'tcx > DefIdVisitor < 'tcx > for ReachableContext < 'tcx > { type Result = () ; fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn visit_def_id (& mut self , def_id : DefId , _kind : & str , _descr : & dyn std :: fmt :: Display ,) -> Self :: Result { self . propagate_item (Res :: Def (self . tcx . def_kind (def_id) , def_id)) } }
    };
}

impl_310!();