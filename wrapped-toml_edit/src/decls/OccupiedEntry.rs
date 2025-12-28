macro_rules! deps {
    () => {
        Key!();
        Table!();
        Item!();
    };
}

macro_rules! OccupiedEntry {
    () => {
        deps!();
        # [doc = " A view into a single occupied location in a [`Table`]."] pub struct OccupiedEntry < 'a > { pub (crate) entry : indexmap :: map :: OccupiedEntry < 'a , Key , Item > , }
    };
}

OccupiedEntry!()