macro_rules! deps {
    () => {
        InternalMarker!();
    };
}

macro_rules! GetBit {
    () => {
        deps!();
        # [allow (missing_docs)] pub trait GetBit < I > { # [allow (missing_docs)] type Output ; # [doc (hidden)] fn get_bit < IM : InternalMarker > (& self , _ : & I) -> Self :: Output ; }
    };
}

GetBit!();