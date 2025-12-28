macro_rules! deps {
    () => {
        OccupiedEntry!();
        InlineTable!();
        Key!();
        Item!();
    };
}

macro_rules! InlineOccupiedEntry {
    () => {
        deps!();
        # [doc = " A view into a single occupied location in an [`InlineTable`]."] pub struct InlineOccupiedEntry < 'a > { entry : indexmap :: map :: OccupiedEntry < 'a , Key , Item > , }
    };
}

InlineOccupiedEntry!()