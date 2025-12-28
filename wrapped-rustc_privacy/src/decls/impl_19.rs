macro_rules! deps {
    () => {
        FindMin!();
        DefIdVisitor!();
        VisibilityLike!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < 'a , 'tcx , VL : VisibilityLike , const SHALLOW : bool > DefIdVisitor < 'tcx > for FindMin < 'a , 'tcx , VL , SHALLOW > { const SHALLOW : bool = SHALLOW ; fn skip_assoc_tys (& self) -> bool { true } fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn visit_def_id (& mut self , def_id : DefId , _kind : & str , _descr : & dyn fmt :: Display) { if let Some (def_id) = def_id . as_local () { self . min = VL :: new_min (self , def_id) ; } } }
    };
}

impl_19!()