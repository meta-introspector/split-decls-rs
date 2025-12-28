macro_rules! deps {
    () => {
        Children!();
        Map!();
    };
}

macro_rules! impl_260 {
    () => {
        deps!();
        impl < 'a , 'tcx > Children < 'a , 'tcx > { fn new (map : & 'a Map < 'tcx > , parent : PlaceIndex) -> Self { Self { map , next : map . places [parent] . first_child } } }
    };
}

impl_260!()