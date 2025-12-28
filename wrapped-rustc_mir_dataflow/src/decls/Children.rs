macro_rules! deps {
    () => {
        Map!();
    };
}

macro_rules! Children {
    () => {
        deps!();
        struct Children < 'a , 'tcx > { map : & 'a Map < 'tcx > , next : Option < PlaceIndex > , }
    };
}

Children!()