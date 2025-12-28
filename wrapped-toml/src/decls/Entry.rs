macro_rules! deps {
    () => {
        VacantEntry!();
        OccupiedEntry!();
        Map!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        # [doc = " A view into a single entry in a map, which may either be vacant or occupied."] # [doc = " This enum is constructed from the [`entry`] method on [`Map`]."] # [doc = ""] # [doc = " [`entry`]: struct.Map.html#method.entry"] # [doc = " [`Map`]: struct.Map.html"] pub enum Entry < 'a , K , V > { # [doc = " A vacant Entry."] Vacant (VacantEntry < 'a , K , V >) , # [doc = " An occupied Entry."] Occupied (OccupiedEntry < 'a , K , V >) , }
    };
}

Entry!();