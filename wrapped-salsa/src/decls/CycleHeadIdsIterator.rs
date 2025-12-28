macro_rules! deps {
    () => {
        CycleHeadsIterator!();
    };
}

macro_rules! CycleHeadIdsIterator {
    () => {
        deps!();
        # [derive (Clone)] pub struct CycleHeadIdsIterator < 'a > { inner : CycleHeadsIterator < 'a > , }
    };
}

CycleHeadIdsIterator!();