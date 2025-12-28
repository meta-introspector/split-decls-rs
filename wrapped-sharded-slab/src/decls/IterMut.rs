macro_rules! deps {
    () => {
        Config!();
        Ptr!();
    };
}

macro_rules! IterMut {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct IterMut < 'a , T : 'a , C : cfg :: Config + 'a > (slice :: IterMut < 'a , Ptr < T , C > >) ;
    };
}

IterMut!();