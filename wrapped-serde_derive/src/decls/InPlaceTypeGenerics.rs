macro_rules! deps {
    () => {
        Parameters!();
    };
}

macro_rules! InPlaceTypeGenerics {
    () => {
        deps!();
        # [cfg (feature = "deserialize_in_place")] struct InPlaceTypeGenerics < 'a > (& 'a Parameters) ;
    };
}

InPlaceTypeGenerics!();