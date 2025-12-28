macro_rules! deps {
    () => {
        MoveVisitor!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for MoveVisitor < '_ , '_ , 'tcx > { fn visit_local (& mut self , local : Local , context : PlaceContext , loc : Location) { if PlaceContext :: NonMutatingUse (NonMutatingUseContext :: Move) == context { self . borrowed_locals . seek_before_primary_effect (loc) ; if ! self . borrowed_locals . get () . contains (local) { self . state . kill (local) ; } } } }
    };
}

impl_177!()