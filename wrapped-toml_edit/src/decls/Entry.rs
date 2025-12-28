macro_rules! deps {
    () => {
        VacantEntry!();
        OccupiedEntry!();
        Table!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        # [doc = " A view into a single location in a [`Table`], which may be vacant or occupied."] pub enum Entry < 'a > { # [doc = " An occupied Entry."] Occupied (OccupiedEntry < 'a >) , # [doc = " A vacant Entry."] Vacant (VacantEntry < 'a >) , }
    };
}

Entry!()