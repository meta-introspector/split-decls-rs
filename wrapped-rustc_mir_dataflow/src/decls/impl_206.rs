macro_rules! deps {
    () => {
        MoveData!();
        MovePathLookup!();
        LocationMap!();
        MoveDataBuilder!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl < 'a , 'tcx , F : Fn (Ty < 'tcx >) -> bool > MoveDataBuilder < 'a , 'tcx , F > { fn new (body : & 'a Body < 'tcx > , tcx : TyCtxt < 'tcx > , filter : F) -> Self { let mut move_paths = IndexVec :: new () ; let mut path_map = IndexVec :: new () ; let mut init_path_map = IndexVec :: new () ; let locals = body . local_decls . iter_enumerated () . map (| (i , l) | { if l . is_deref_temp () { return None ; } if filter (l . ty) { Some (new_move_path (& mut move_paths , & mut path_map , & mut init_path_map , None , Place :: from (i) ,)) } else { None } }) . collect () ; MoveDataBuilder { body , loc : Location :: START , tcx , data : MoveData { moves : IndexVec :: new () , loc_map : LocationMap :: new (body) , rev_lookup : MovePathLookup { locals , projections : Default :: default () , un_derefer : Default :: default () , } , move_paths , path_map , inits : IndexVec :: new () , init_loc_map : LocationMap :: new (body) , init_path_map , } , filter , } } }
    };
}

impl_206!();