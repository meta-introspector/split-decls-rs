macro_rules! deps {
    () => {
        NestedNormalizationGoals!();
        GenericArg!();
        Interner!();
        OutlivesPredicate!();
        Ty!();
        OpaqueTypeKey!();
    };
}

macro_rules! ExternalConstraintsData {
    () => {
        deps!();
        # [doc = " Additional constraints returned on success."] # [derive_where (Clone , Hash , PartialEq , Debug , Default ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic)] # [cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] pub struct ExternalConstraintsData < I : Interner > { pub region_constraints : Vec < ty :: OutlivesPredicate < I , I :: GenericArg > > , pub opaque_types : Vec < (ty :: OpaqueTypeKey < I > , I :: Ty) > , pub normalization_nested_goals : NestedNormalizationGoals < I > , }
    };
}

ExternalConstraintsData!()