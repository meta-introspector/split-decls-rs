macro_rules! IsThirPolymorphic {
    () => {
        struct IsThirPolymorphic < 'a , 'tcx > { is_poly : bool , thir : & 'a thir :: Thir < 'tcx > , }
    };
}

IsThirPolymorphic!();