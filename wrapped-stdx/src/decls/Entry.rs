macro_rules! deps {
    () => {
        Downcast!();
        VacantEntry!();
        OccupiedEntry!();
        Map!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        # [doc = " A view into a single location in an `Map`, which may be vacant or occupied."] pub enum Entry < 'map , A : ? Sized + Downcast , V > { # [doc = " An occupied Entry"] Occupied (OccupiedEntry < 'map , A , V >) , # [doc = " A vacant Entry"] Vacant (VacantEntry < 'map , A , V >) , }
    };
}

Entry!();