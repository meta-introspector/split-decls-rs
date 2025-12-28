macro_rules! deps {
    () => {
        CycleHeadIdsIterator!();
        Id!();
    };
}

macro_rules! Cycle {
    () => {
        deps!();
        # [doc = " The context that the cycle recovery function receives when a query cycle occurs."] pub struct Cycle < 'a > { pub (crate) head_ids : CycleHeadIdsIterator < 'a > , pub (crate) id : Id , pub (crate) iteration : u32 , }
    };
}

Cycle!()