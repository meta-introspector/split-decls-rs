macro_rules! deps {
    () => {
        BorrowedFieldProjectionMode!();
        PointerFinder!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < 'a , 'tcx > PointerFinder < 'a , 'tcx > { fn new (tcx : TyCtxt < 'tcx > , local_decls : & 'a mut LocalDecls < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , excluded_pointees : & 'a [Ty < 'tcx >] , field_projection_mode : BorrowedFieldProjectionMode ,) -> Self { PointerFinder { tcx , local_decls , typing_env , excluded_pointees , pointers : Vec :: new () , field_projection_mode , } } fn into_found_pointers (self) -> Vec < (Place < 'tcx > , Ty < 'tcx > , PlaceContext) > { self . pointers } # [doc = " Whether or not we should visit a [Place] with [PlaceContext]."] # [doc = ""] # [doc = " We generally only visit Reads/Writes to a place and only Borrows if"] # [doc = " requested."] fn should_visit_place (& self , context : PlaceContext) -> bool { match context { PlaceContext :: MutatingUse (MutatingUseContext :: Store | MutatingUseContext :: Call | MutatingUseContext :: Yield | MutatingUseContext :: Drop | MutatingUseContext :: Borrow ,) => true , PlaceContext :: NonMutatingUse (NonMutatingUseContext :: Copy | NonMutatingUseContext :: Move | NonMutatingUseContext :: SharedBorrow ,) => true , _ => false , } } }
    };
}

impl_21!();