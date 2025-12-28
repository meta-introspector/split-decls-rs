macro_rules! deps {
    () => {
        TypeFoldable!();
        Interner!();
        RegionFolder!();
        Region!();
    };
}

macro_rules! fold_regions {
    () => {
        deps!();
        pub fn fold_regions < I : Interner , T > (cx : I , value : T , f : impl FnMut (I :: Region , ty :: DebruijnIndex) -> I :: Region ,) -> T where T : TypeFoldable < I > , { value . fold_with (& mut RegionFolder :: new (cx , f)) }
    };
}

fold_regions!()