macro_rules! deps {
    () => {
        Tys!();
        Interner!();
    };
}

macro_rules! CoroutineWitnessTypes {
    () => {
        deps!();
        # [derive_where (Clone , Copy , Debug , PartialEq , Hash ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] pub struct CoroutineWitnessTypes < I : Interner > { pub types : I :: Tys , pub assumptions : I :: RegionAssumptions , }
    };
}

CoroutineWitnessTypes!();