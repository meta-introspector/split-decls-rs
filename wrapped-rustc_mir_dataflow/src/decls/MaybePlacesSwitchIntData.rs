macro_rules! MaybePlacesSwitchIntData {
    () => {
        pub struct MaybePlacesSwitchIntData < 'tcx > { enum_place : mir :: Place < 'tcx > , discriminants : Vec < (VariantIdx , Discr < 'tcx >) > , index : usize , }
    };
}

MaybePlacesSwitchIntData!()