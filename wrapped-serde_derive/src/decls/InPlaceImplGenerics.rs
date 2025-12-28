macro_rules! deps {
    () => {
        Parameters!();
    };
}

macro_rules! InPlaceImplGenerics {
    () => {
        deps!();
        # [cfg (feature = "deserialize_in_place")] struct InPlaceImplGenerics < 'a > (& 'a Parameters) ;
    };
}

InPlaceImplGenerics!();