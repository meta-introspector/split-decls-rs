macro_rules! deps {
    () => {
        TrackElem!();
    };
}

macro_rules! impl_265 {
    () => {
        deps!();
        impl < V , T > TryFrom < ProjectionElem < V , T > > for TrackElem { type Error = () ; fn try_from (value : ProjectionElem < V , T >) -> Result < Self , Self :: Error > { match value { ProjectionElem :: Field (field , _) => Ok (TrackElem :: Field (field)) , ProjectionElem :: Downcast (_ , idx) => Ok (TrackElem :: Variant (idx)) , _ => Err (()) , } } }
    };
}

impl_265!()