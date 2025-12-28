macro_rules! deps {
    () => {
        Entry!();
        OccupiedEntryImpl!();
    };
}

macro_rules! OccupiedEntry {
    () => {
        deps!();
        # [doc = " An occupied Entry. It is part of the [`Entry`] enum."] # [doc = ""] # [doc = " [`Entry`]: enum.Entry.html"] pub struct OccupiedEntry < 'a , K , V > { occupied : OccupiedEntryImpl < 'a , K , V > , }
    };
}

OccupiedEntry!()