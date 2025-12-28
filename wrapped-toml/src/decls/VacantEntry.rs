macro_rules! deps {
    () => {
        Entry!();
        VacantEntryImpl!();
    };
}

macro_rules! VacantEntry {
    () => {
        deps!();
        # [doc = " A vacant Entry. It is part of the [`Entry`] enum."] # [doc = ""] # [doc = " [`Entry`]: enum.Entry.html"] pub struct VacantEntry < 'a , K , V > { vacant : VacantEntryImpl < 'a , K , V > , }
    };
}

VacantEntry!();