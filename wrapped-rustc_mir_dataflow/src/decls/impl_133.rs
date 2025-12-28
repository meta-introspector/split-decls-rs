macro_rules! deps {
    () => {
        MaybePlacesSwitchIntData!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl < 'tcx > MaybePlacesSwitchIntData < 'tcx > { # [doc = " Creates a `SmallVec` mapping each target in `targets` to its `VariantIdx`."] fn variants (& mut self , targets : & mir :: SwitchTargets) -> SmallVec < [VariantIdx ; 4] > { self . index = 0 ; targets . all_values () . iter () . map (| value | self . next_discr (value . get ())) . collect () } fn next_discr (& mut self , value : u128) -> VariantIdx { loop { let (variant , discr) = self . discriminants [self . index] ; self . index += 1 ; if discr . val == value { return variant ; } } } }
    };
}

impl_133!();