macro_rules! deps {
    () => {
        Region!();
        Interner!();
    };
}

macro_rules! OutlivesPredicate {
    () => {
        deps!();
        # [doc = " `A: 'region`"] # [derive_where (Clone , Hash , PartialEq , Debug ; I : Interner , A)] # [derive_where (Copy ; I : Interner , A : Copy)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct OutlivesPredicate < I : Interner , A > (pub A , pub I :: Region) ;
    };
}

OutlivesPredicate!()