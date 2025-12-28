macro_rules! deps {
    () => {
        PointerFinder!();
        BorrowedFieldProjectionMode!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < 'a , 'tcx > Visitor < 'tcx > for PointerFinder < 'a , 'tcx > { fn visit_place (& mut self , place : & Place < 'tcx > , context : PlaceContext , location : Location) { if ! self . should_visit_place (context) || ! place . is_indirect () { return ; } let pointer = Place :: from (place . local) ; let pointer_ty = pointer . ty (self . local_decls , self . tcx) . ty ; let & ty :: RawPtr (mut pointee_ty , _) = pointer_ty . kind () else { trace ! ("Indirect, but not based on an raw ptr, not checking {:?}" , place) ; return ; } ; if matches ! (self . field_projection_mode , BorrowedFieldProjectionMode :: FollowProjections) && matches ! (context , PlaceContext :: NonMutatingUse (NonMutatingUseContext :: SharedBorrow) | PlaceContext :: MutatingUse (MutatingUseContext :: Borrow)) { pointee_ty = place . ty (self . local_decls , self . tcx) . ty ; } if ! pointee_ty . is_sized (self . tcx , self . typing_env) { trace ! ("Raw pointer, but pointee is not known to be sized: {:?}" , pointer_ty) ; return ; } let element_ty = match pointee_ty . kind () { ty :: Array (ty , _) => * ty , _ => pointee_ty , } ; if self . excluded_pointees . contains (& element_ty) { trace ! ("Skipping pointer for type: {:?}" , pointee_ty) ; return ; } self . pointers . push ((pointer , pointee_ty , context)) ; self . super_place (place , context , location) ; } }
    };
}

impl_22!();