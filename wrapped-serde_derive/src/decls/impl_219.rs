macro_rules! deps {
    () => {
        InPlaceImplGenerics!();
        DeImplGenerics!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        # [cfg (feature = "deserialize_in_place")] impl < 'a > DeImplGenerics < 'a > { fn in_place (self) -> InPlaceImplGenerics < 'a > { InPlaceImplGenerics (self . 0) } }
    };
}

impl_219!();