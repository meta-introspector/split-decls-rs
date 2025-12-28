macro_rules! deps {
    () => {
        InlineTable!();
        Key!();
        VacantEntry!();
        Item!();
    };
}

macro_rules! InlineVacantEntry {
    () => {
        deps!();
        # [doc = " A view into a single empty location in an [`InlineTable`]."] pub struct InlineVacantEntry < 'a > { entry : indexmap :: map :: VacantEntry < 'a , Key , Item > , }
    };
}

InlineVacantEntry!();