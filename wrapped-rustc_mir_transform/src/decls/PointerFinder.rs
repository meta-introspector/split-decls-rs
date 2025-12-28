macro_rules! deps {
    () => {
        BorrowedFieldProjectionMode!();
    };
}

macro_rules! PointerFinder {
    () => {
        deps!();
        struct PointerFinder < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , local_decls : & 'a mut LocalDecls < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , pointers : Vec < (Place < 'tcx > , Ty < 'tcx > , PlaceContext) > , excluded_pointees : & 'a [Ty < 'tcx >] , field_projection_mode : BorrowedFieldProjectionMode , }
    };
}

PointerFinder!();