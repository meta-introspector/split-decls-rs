macro_rules! deps {
    () => {
        InPlaceTypeGenerics!();
        DeTypeGenerics!();
    };
}

macro_rules! impl_225 {
    () => {
        deps!();
        # [cfg (feature = "deserialize_in_place")] impl < 'a > DeTypeGenerics < 'a > { fn in_place (self) -> InPlaceTypeGenerics < 'a > { InPlaceTypeGenerics (self . 0) } }
    };
}

impl_225!()