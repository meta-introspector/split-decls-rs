macro_rules! deps {
    () => {
        Analysis!();
        MaybeRequiresStorage!();
        MoveVisitor!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl < 'tcx > MaybeRequiresStorage < '_ , 'tcx > { # [doc = " Kill locals that are fully moved and have not been borrowed."] fn check_for_move (& mut self , state : & mut < Self as Analysis < 'tcx > > :: Domain , loc : Location) { let body = self . borrowed_locals . body () ; let mut visitor = MoveVisitor { state , borrowed_locals : & mut self . borrowed_locals } ; visitor . visit_location (body , loc) ; } }
    };
}

impl_175!();