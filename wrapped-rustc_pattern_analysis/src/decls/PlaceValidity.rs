macro_rules! PlaceValidity {
    () => {
        # [doc = " Track whether a given place (aka column) is known to contain a valid value or not."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum PlaceValidity { ValidOnly , MaybeInvalid , }
    };
}

PlaceValidity!();