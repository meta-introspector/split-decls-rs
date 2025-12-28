macro_rules! deps {
    () => {
        RegionKind!();
        Interner!();
        Flags!();
        Relate!();
        IntoKind!();
        GenericArg!();
    };
}

macro_rules! Region {
    () => {
        deps!();
        pub trait Region < I : Interner < Region = Self > > : Copy + Debug + Hash + Eq + Into < I :: GenericArg > + IntoKind < Kind = ty :: RegionKind < I > > + Flags + Relate < I > { fn new_bound (interner : I , debruijn : ty :: DebruijnIndex , var : I :: BoundRegion) -> Self ; fn new_anon_bound (interner : I , debruijn : ty :: DebruijnIndex , var : ty :: BoundVar) -> Self ; fn new_static (interner : I) -> Self ; fn new_placeholder (interner : I , var : I :: PlaceholderRegion) -> Self ; fn is_bound (self) -> bool { matches ! (self . kind () , ty :: ReBound (..)) } }
    };
}

Region!()