macro_rules! deps {
    () => {
        IngredientIndex!();
        PageIndex!();
        Page!();
    };
}

macro_rules! Table {
    () => {
        deps!();
        pub struct Table { pages : boxcar :: Vec < Page > , # [doc = " Map from ingredient to non-full pages that are up for grabs"] non_full_pages : Mutex < FxHashMap < IngredientIndex , Vec < PageIndex > > > , }
    };
}

Table!()