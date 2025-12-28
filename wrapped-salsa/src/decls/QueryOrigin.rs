macro_rules! deps {
    () => {
        IngredientIndex!();
        QueryRevisions!();
        Id!();
        QueryOriginKind!();
        DatabaseKeyIndex!();
    };
}

macro_rules! QueryOrigin {
    () => {
        deps!();
        # [doc = " Tracks how a memoized value for a given query was created."] # [doc = ""] # [doc = " This type is a manual enum packed to 13 bytes to reduce the size of `QueryRevisions`."] # [repr (Rust , packed)] pub struct QueryOrigin { # [doc = " The tag of this enum."] # [doc = ""] # [doc = " Note that this tag only requires two bits and could likely be packed into"] # [doc = " some other field. However, we get this byte for free thanks to alignment."] kind : QueryOriginKind , # [doc = " The data portion of this enum."] data : QueryOriginData , # [doc = " The metadata of this enum."] # [doc = ""] # [doc = " For `QueryOriginKind::Derived` and `QueryOriginKind::DerivedUntracked`, this"] # [doc = " is the length of the `input_outputs` allocation."] # [doc = ""] # [doc = " For `QueryOriginKind::Assigned`, this is the `IngredientIndex` of assigning query."] # [doc = " Combined with the `Id` data, this forms a complete `DatabaseKeyIndex`."] # [doc = ""] # [doc = " For `QueryOriginKind::FixpointInitial`, this field is zero."] metadata : u32 , }
    };
}

QueryOrigin!();