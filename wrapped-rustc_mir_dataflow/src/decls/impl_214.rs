macro_rules! deps {
    () => {
        MovePathLookup!();
        LookupResult!();
    };
}

macro_rules! impl_214 {
    () => {
        deps!();
        impl < 'tcx > MovePathLookup < 'tcx > { pub fn find (& self , place : PlaceRef < 'tcx >) -> LookupResult { let Some (mut result) = self . find_local (place . local) else { return LookupResult :: Parent (None) ; } ; for (_ , elem) in self . un_derefer . iter_projections (place) { if let Some (& subpath) = self . projections . get (& (result , elem . kind ())) { result = subpath ; } else { return LookupResult :: Parent (Some (result)) ; } } LookupResult :: Exact (result) } # [inline] pub fn find_local (& self , local : Local) -> Option < MovePathIndex > { self . locals [local] } # [doc = " An enumerated iterator of `local`s and their associated"] # [doc = " `MovePathIndex`es."] pub fn iter_locals_enumerated (& self ,) -> impl DoubleEndedIterator < Item = (Local , MovePathIndex) > { self . locals . iter_enumerated () . filter_map (| (l , & idx) | Some ((l , idx ?))) } }
    };
}

impl_214!();