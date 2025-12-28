macro_rules! deps {
    () => {
        CycleHead!();
    };
}

macro_rules! CycleHeadsIterator {
    () => {
        deps!();
        # [derive (Clone)] pub struct CycleHeadsIterator < 'a > { inner : std :: slice :: Iter < 'a , CycleHead > , }
    };
}

CycleHeadsIterator!()