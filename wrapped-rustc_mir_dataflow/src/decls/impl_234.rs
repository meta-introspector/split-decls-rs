macro_rules! deps {
    () => {
        ProjectionIter!();
        UnDerefer!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl < 'tcx > UnDerefer < 'tcx > { # [inline] pub (crate) fn insert (& mut self , local : Local , reffed : PlaceRef < 'tcx >) { let mut chain = self . deref_chains . remove (& reffed . local) . unwrap_or_default () ; chain . push (reffed) ; self . deref_chains . insert (local , chain) ; } # [doc = " Returns the chain of places behind `DerefTemp` locals"] # [inline] pub (crate) fn deref_chain (& self , local : Local) -> & [PlaceRef < 'tcx >] { self . deref_chains . get (& local) . map (Vec :: as_slice) . unwrap_or_default () } # [doc = " Iterates over the projections of a place and its deref chain."] # [doc = ""] # [doc = " See [`PlaceRef::iter_projections`]"] # [inline] pub (crate) fn iter_projections (& self , place : PlaceRef < 'tcx > ,) -> impl Iterator < Item = (PlaceRef < 'tcx > , PlaceElem < 'tcx >) > { ProjectionIter :: new (self . deref_chain (place . local) , place) } }
    };
}

impl_234!()