macro_rules! deps {
    () => {
        Component!();
        Ty!();
        Interner!();
    };
}

macro_rules! OutlivesCollector {
    () => {
        deps!();
        struct OutlivesCollector < 'a , I : Interner > { cx : I , out : & 'a mut SmallVec < [Component < I > ; 4] > , visited : SsoHashSet < I :: Ty > , }
    };
}

OutlivesCollector!()