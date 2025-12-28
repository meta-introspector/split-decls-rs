macro_rules! deps {
    () => {
        Entry!();
        InlineTable!();
        InlineOccupiedEntry!();
        InlineVacantEntry!();
    };
}

macro_rules! InlineEntry {
    () => {
        deps!();
        # [doc = " A view into a single location in an [`InlineTable`], which may be vacant or occupied."] pub enum InlineEntry < 'a > { # [doc = " An occupied Entry."] Occupied (InlineOccupiedEntry < 'a >) , # [doc = " A vacant Entry."] Vacant (InlineVacantEntry < 'a >) , }
    };
}

InlineEntry!()