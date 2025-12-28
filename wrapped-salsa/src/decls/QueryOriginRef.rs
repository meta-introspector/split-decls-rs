macro_rules! deps {
    () => {
        QueryOrigin!();
        QueryEdge!();
        QueryOriginKind!();
        DatabaseKeyIndex!();
    };
}

macro_rules! QueryOriginRef {
    () => {
        deps!();
        # [doc = " Tracks the way that a memoized value for a query was created."] # [doc = ""] # [doc = " This is a read-only reference to a `PackedQueryOrigin`."] # [repr (u8)] # [derive (Debug , Clone , Copy)] # [cfg_attr (feature = "persistence" , derive (serde :: Serialize))] # [cfg_attr (feature = "persistence" , serde (rename = "QueryOrigin"))] pub enum QueryOriginRef < 'a > { # [doc = " The value was assigned as the output of another query (e.g., using `specify`)."] # [doc = " The `DatabaseKeyIndex` is the identity of the assigning query."] Assigned (DatabaseKeyIndex) = QueryOriginKind :: Assigned as u8 , # [doc = " The value was derived by executing a function"] # [doc = " and we were able to track ALL of that function's inputs."] # [doc = " Those inputs are described in [`QueryEdges`]."] Derived (& 'a [QueryEdge]) = QueryOriginKind :: Derived as u8 , # [doc = " The value was derived by executing a function"] # [doc = " but that function also reported that it read untracked inputs."] # [doc = " The [`QueryEdges`] argument contains a listing of all the inputs we saw"] # [doc = " (but we know there were more)."] DerivedUntracked (& 'a [QueryEdge]) = QueryOriginKind :: DerivedUntracked as u8 , # [doc = " The value is an initial provisional value for a query that supports fixpoint iteration."] FixpointInitial = QueryOriginKind :: FixpointInitial as u8 , }
    };
}

QueryOriginRef!()