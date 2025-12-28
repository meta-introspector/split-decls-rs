macro_rules! deps {
    () => {
        Stable!();
        BoundRegion!();
        BridgeTys!();
        RegionKind!();
        EarlyParamRegion!();
        Placeholder!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: RegionKind < 'tcx > { type T = crate :: ty :: RegionKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: { BoundRegion , EarlyParamRegion , RegionKind } ; match self { ty :: ReEarlyParam (early_reg) => RegionKind :: ReEarlyParam (EarlyParamRegion { index : early_reg . index , name : early_reg . name . to_string () , }) , ty :: ReBound (db_index , bound_reg) => RegionKind :: ReBound (db_index . as_u32 () , BoundRegion { var : bound_reg . var . as_u32 () , kind : bound_reg . kind . stable (tables , cx) , } ,) , ty :: ReStatic => RegionKind :: ReStatic , ty :: RePlaceholder (place_holder) => RegionKind :: RePlaceholder (crate :: ty :: Placeholder { universe : place_holder . universe . as_u32 () , bound : BoundRegion { var : place_holder . bound . var . as_u32 () , kind : place_holder . bound . kind . stable (tables , cx) , } , }) , ty :: ReErased => RegionKind :: ReErased , _ => unreachable ! ("{self:?}") , } } }
    };
}

impl_211!()