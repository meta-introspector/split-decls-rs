macro_rules! deps {
    () => {
        CycleHeadIdsIterator!();
        Id!();
        Cycle!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl Cycle < '_ > { # [doc = " An iterator that outputs the [`Id`]s of the current cycle heads."] # [doc = " This always contains the [`Id`] of the current query but it can contain additional cycle head [`Id`]s"] # [doc = " if this query is nested in an outer cycle or if it has nested cycles."] pub fn head_ids (& self) -> CycleHeadIdsIterator < '_ > { self . head_ids . clone () } # [doc = " The [`Id`] of the query that the current cycle recovery function is processing."] pub fn id (& self) -> Id { self . id } # [doc = " The counter of the current fixed point iteration."] pub fn iteration (& self) -> u32 { self . iteration } }
    };
}

impl_69!();