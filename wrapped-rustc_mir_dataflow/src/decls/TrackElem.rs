macro_rules! TrackElem {
    () => {
        # [doc = " The set of projection elements that can be used by a tracked place."] # [doc = ""] # [doc = " Although only field projections are currently allowed, this could change in the future."] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] pub enum TrackElem { Field (FieldIdx) , Variant (VariantIdx) , Discriminant , DerefLen , }
    };
}

TrackElem!()