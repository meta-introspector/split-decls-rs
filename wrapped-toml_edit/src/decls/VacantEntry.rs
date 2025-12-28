macro_rules! deps {
    () => {
        Table!();
        Key!();
        Item!();
    };
}

macro_rules! VacantEntry {
    () => {
        deps!();
        # [doc = " A view into a single empty location in a [`Table`]."] pub struct VacantEntry < 'a > { pub (crate) entry : indexmap :: map :: VacantEntry < 'a , Key , Item > , }
    };
}

VacantEntry!()