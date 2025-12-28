macro_rules! deps {
    () => {
        TrackedEntry!();
        Id!();
        Identity!();
    };
}

macro_rules! IdentityMap {
    () => {
        deps!();
        # [doc = " A map from tracked struct [`Identity`] to their final [`Id`]."] # [derive (Default , Debug)] pub (crate) struct IdentityMap { table : hashbrown :: HashTable < TrackedEntry > , }
    };
}

IdentityMap!()