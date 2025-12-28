macro_rules! deps {
    () => {
        OccupiedEntry!();
        Item!();
        InlineTable!();
        Key!();
    };
}

macro_rules! InlineOccupiedEntry {
    () => {
        deps!();
        # [doc = " A view into a single occupied location in an [`InlineTable`]."] pub struct InlineOccupiedEntry < 'a > { entry : indexmap :: map :: OccupiedEntry < 'a , Key , Item > , }
    };
}

InlineOccupiedEntry!();