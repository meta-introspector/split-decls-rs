macro_rules! deps {
    () => {
        BoundRegion!();
        EarlyParamRegion!();
        DebruijnIndex!();
        Placeholder!();
    };
}

macro_rules! RegionKind {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum RegionKind { ReEarlyParam (EarlyParamRegion) , ReBound (DebruijnIndex , BoundRegion) , ReStatic , RePlaceholder (Placeholder < BoundRegion >) , ReErased , }
    };
}

RegionKind!()